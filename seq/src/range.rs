//! 统一 Range<usize> 和 RangeInclusive<usize> 两种类型

pub type RangeLit = usize;
type StdRange = std::ops::Range<RangeLit>;
type StdRangeInc = std::ops::RangeInclusive<RangeLit>;

#[derive(Clone)]
pub enum Range {
    Normal(StdRange),
    Inclusive(StdRangeInc),
}

impl Iterator for Range {
    type Item = RangeLit;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Range::Normal(i) => i.next(),
            Range::Inclusive(i) => i.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Range::Normal(i) => i.size_hint(),
            Range::Inclusive(i) => i.size_hint(),
        }
    }
}

impl From<StdRange> for Range {
    fn from(r: StdRange) -> Self {
        Range::Normal(r)
    }
}

impl From<StdRangeInc> for Range {
    fn from(r: StdRangeInc) -> Self {
        Range::Inclusive(r)
    }
}
