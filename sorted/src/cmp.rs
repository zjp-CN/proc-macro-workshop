use std::ops::ControlFlow;

type StrPair<'s> = (&'s String, &'s String);

fn cmp_str((sorted, raw): StrPair) -> ControlFlow<StrPair> {
    if sorted != raw {
        ControlFlow::Break((sorted, raw))
    } else {
        ControlFlow::Continue(())
    }
}

pub fn finish<F: FnOnce(usize) -> proc_macro2::Span>(raw: Vec<String>, f: F) -> syn::Result<()> {
    let mut sorted = raw.clone();
    sorted.sort();
    let (mut raw, sorted) = (raw.iter(), sorted.iter());
    if let ControlFlow::Break((sorted_s, raw_s)) = sorted.zip(raw.clone()).try_for_each(cmp_str) {
        let pos = raw.position(|i| i == sorted_s).unwrap(); // 一定能找到
        Err(syn::Error::new((f)(pos), format!("{} should sort before {}", sorted_s, raw_s)))
    } else {
        Ok(())
    }
}
