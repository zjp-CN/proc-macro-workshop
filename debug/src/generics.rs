use std::collections::HashSet;
use syn::{parse_quote, Ident, Type};

pub fn add_debug<'f>(ty: &mut syn::TypeParam, field_ty: impl Iterator<Item = &'f Type>,
                     associated: &mut HashSet<&'f Type>, bound: &HashSet<Ident>) {
    let syn::TypeParam { ref ident, bounds, .. } = ty;
    let phantom_data: Type = parse_quote!(PhantomData<#ident>);
    // do not add Debug trait constrain
    // when the generics T contains associated types or T is PhantomData<T> or
    // `T::Associated: Debug` is in bound
    if !field_ty.fold(bound.contains(ident), |acc, t| {
                    search(t, ident, associated) || t == &phantom_data || acc
                })
    {
        bounds.push(parse_quote!(::std::fmt::Debug));
    }
}

// 处理字段类型的关联类型
fn search<'f>(ty: &'f Type, ident: &Ident, associated: &mut HashSet<&'f Type>) -> bool {
    use syn::{AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, TypePath};

    // 把 T::Associated 添加到 where 语句增加项
    fn check_associated<'f>(ty: &'f Type, ident: &Ident, associated: &mut HashSet<&'f Type>) -> bool {
        if let Type::Path(TypePath { path: Path { segments, leading_colon: None }, .. }) = ty {
            if segments.len() > 1 && segments.first().map(|seg| &seg.ident == ident).unwrap_or(false) {
                associated.insert(ty);
                return true;
            }
        }
        false
    }

    // 一层尖括号泛型中的关联类型 path::<T::Associated>
    fn check_angle_bracket_associated<'f>(ty: &'f Type, ident: &Ident, associated: &mut HashSet<&'f Type>)
                                          -> bool {
        // 检查尖括号内的泛型是否为关联类型
        fn check<'f>(arg: &'f PathArguments, ident: &Ident, associated: &mut HashSet<&'f Type>) -> bool {
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = arg {
                args.iter().fold(false, |acc, arg| {
                               if let GenericArgument::Type(t) = arg {
                                   check_associated(t, ident, associated) || acc
                               } else {
                                   acc
                               }
                           })
            } else {
                false
            }
        }
        if let Type::Path(TypePath { path: Path { segments, .. }, .. }) = ty {
            // 只考虑最后路径上的泛型，即 a::b::c::<T, I::Item, ...> 形式
            return segments.last()
                           .map(|seg| check(&seg.arguments, ident, associated))
                           .unwrap_or(false);
        }
        false
    }

    check_associated(ty, ident, associated) || check_angle_bracket_associated(ty, ident, associated)
}
