#[derive(Debug, Clone)]
pub struct SegmentTree<'s> {
    pub segments: Vec<Segment<'s>>,
}

#[derive(Debug, Clone)]
pub enum Segment<'s> {
    Text(&'s str),
    Not(SegmentTree<'s>),
    Loop {
        count: SegmentTree<'s>,
        contents: SegmentTree<'s>,
    },
    If {
        condition: SegmentTree<'s>,
        contents: SegmentTree<'s>,
    },
    Variable {
        name: SegmentTree<'s>,
    },
    With {
        assignments: Vec<(SegmentTree<'s>, SegmentTree<'s>)>,
        contents: SegmentTree<'s>,
    },
}
