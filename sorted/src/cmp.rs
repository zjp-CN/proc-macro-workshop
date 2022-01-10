use std::ops::ControlFlow;

type StrPair<'s> = (&'s String, &'s String);

fn cmp_str((sorted, raw): StrPair) -> ControlFlow<StrPair> {
    if sorted != raw {
        ControlFlow::Break((sorted, raw))
    } else {
        ControlFlow::Continue(())
    }
}

pub struct StringCmp<F: FnOnce(usize) -> proc_macro2::Span> {
    f:      F,
    raw:    Vec<String>,
    sorted: Vec<String>,
}

impl<F: FnOnce(usize) -> proc_macro2::Span> StringCmp<F> {
    pub fn new(raw: Vec<String>, f: F) -> Self {
        let mut sorted = raw.clone();
        sorted.sort();
        Self { f, raw, sorted }
    }

    pub fn check(self) -> syn::Result<()> {
        let (mut raw, sorted) = (self.raw.iter(), self.sorted.iter());
        if let ControlFlow::Break((sorted_s, raw_s)) = sorted.zip(raw.clone()).try_for_each(cmp_str) {
            let pos = raw.position(|i| i == sorted_s).unwrap(); // 一定能找到
                                                                // TODO: sorted_s raw_s
            Err(syn::Error::new((self.f)(pos), format!("{} should sort before {}", sorted_s, raw_s)))
        } else {
            Ok(())
        }
    }
}
