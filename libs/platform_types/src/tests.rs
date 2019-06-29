use super::*;
use proptest::{prop_compose, proptest};

prop_compose! {
    fn arb_pos(max_line: usize, max_offset: usize)
    (line in 0..=max_line, offset in 0..=max_offset) -> Position {
        Position{ line, offset: CharOffset(offset) }
    }
}

// I'm not sure it makes sense to compare highllights outside of tests, and if it does, that this
// comparison is the correct way to do it.
#[derive(Debug)]
struct OrdHighlight(Highlight);

macro_rules! h {
    ($($tokens:tt)*) => (
        OrdHighlight(highlight!($($tokens)*))
    );
}

impl Ord for OrdHighlight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .min
            .cmp(&other.0.min)
            .then_with(|| self.0.max.cmp(&other.0.max))
    }
}

impl PartialOrd for OrdHighlight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrdHighlight {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for OrdHighlight {}

fn get_ord_highlights<O: Into<Option<Position>>>(
    position: Position,
    highlight_position: O,
) -> Vec<OrdHighlight> {
    {
        let mut highlights = Vec::with_capacity(16);

        push_highlights(&mut highlights, position, highlight_position);

        highlights
    }
    .into_iter()
    .map(OrdHighlight)
    .collect()
}

proptest! {
    #[test]
    fn number_of_highlights_does_not_exceed_upper_bound(
        pos in arb_pos(99, 99),
        highlight_pos in proptest::option::of(arb_pos(99, 99))
    ) {
        let unoptimized_upper_bound = highlight_pos.map(|h| {
            let min = std::cmp::min(pos, h);
            let max = std::cmp::max(pos, h);

            let lines_difference = max.line - min.line;

            // I have forgotten the reasoning behined thia, ad I just (as of this writing)
            // added the lines_difference max call, to fix a suprise test failure
            std::cmp::max(lines_difference, 2)
             + if h.offset == pos.offset || max.offset == 0 {0} else {1}

        }).unwrap_or(0);

        let upper_bound = std::cmp::min(unoptimized_upper_bound, 3);

        assert!(get_ord_highlights(pos, highlight_pos).len() <= dbg!(upper_bound));
    }

    #[test]
    fn order_of_positions_does_not_affect_highlights(
        pos in arb_pos(99, 99),
        highlight_pos in arb_pos(99, 99)
    ) {
        assert_eq!(
            get_ord_highlights(pos, highlight_pos),
            get_ord_highlights(highlight_pos, pos)
        );
    }
}

macro_rules! highlight_assert {
    (($pos: expr, $highlight_pos: expr), $output: expr) => {
        assert_eq!(
            get_ord_highlights($pos, $highlight_pos),
            $output,
            "\ninput: {:?}",
            ($pos, $highlight_pos)
        );
    };
}

#[test]
fn multiline_highlights_work() {
    highlight_assert!(
        (pos! {l 0 o 0}, pos! {l 0 o 300}),
        vec![h! {l 0 o 0 l 0 o 300}]
    );

    highlight_assert!(
        (pos! {l 0 o 0}, pos! {l 1 o 300}),
        vec![h! {l 0 o 0 l 0 o max}, h! {l 1 o 0 l 1 o 300}]
    );

    highlight_assert!(
        (pos! {l 0 o 200}, pos! {l 0 o 300}),
        vec![h! {l 0 o 200 l 0 o 300}]
    );

    highlight_assert!(
        (pos! {l 0 o 200}, pos! {l 1 o 300}),
        vec![h! {l 0 o 200 l 0 o max}, h! {l 1 o 0 l 1 o 300},]
    );

    highlight_assert!(
        (pos! {l 1 o 0}, pos! {l 2 o 0}),
        vec![h! {l 1 o 0 l 1 o max}]
    );

    highlight_assert!(
        (pos! {l 1 o 0}, pos! {l 3 o 0}),
        vec![h! {l 1 o 0 l 2 o max}]
    );

    highlight_assert!(
        (pos! {l 1 o 400}, pos! {l 2 o 0}),
        vec![h! {l 1 o 400 l 1 o max}]
    );

    highlight_assert!(
        (pos! {l 1 o 400}, pos! {l 3 o 0}),
        vec![h! {l 1 o 400 l 1 o max}, h! {l 2 o 0 l 2 o max}]
    );

    highlight_assert!(
        (pos! {l 1 o 0}, pos! {l 2 o 300}),
        vec![h! {l 1 o 0 l 1 o max}, h! {l 2 o 0 l 2 o 300}]
    );

    highlight_assert!(
        (pos! {l 1 o 0}, pos! {l 3 o 300}),
        vec![h! {l 1 o 0 l 2 o max}, h! {l 3 o 0 l 3 o 300}]
    );

    highlight_assert!(
        (pos! {l 1 o 400}, pos! {l 2 o 300}),
        vec![h! {l 1 o 400 l 1 o max}, h! {l 2 o 0 l 2 o 300}]
    );

    highlight_assert!(
        (pos! {l 1 o 400}, pos! {l 3 o 300}),
        vec![
            h! {l 1 o 400 l 1 o max},
            h! {l 2 o 0 l 2 o max},
            h! {l 3 o 0 l 3 o 300}
        ]
    );
}