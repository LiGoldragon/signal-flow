#![cfg(feature = "datom")]

use datom_codec::{
    Actualizing, Budget, Composable, DatomForming, Datomizable, Potential, Variantizing,
};
use protos::{Compactable, Protosizable, ReaderBudget};
use signal_flow::Presentation;

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    }
}

#[test]
fn presentation_is_one_title_and_escapes_delimiters() {
    let value = Presentation {
        title: "A title with » and trailing \\".into(),
    };
    let text = value
        .datomize(vec![1])
        .named_variant(vec![], "Presentation")
        .protosize()
        .compact();
    assert_eq!(text, r"Presentation.{ «A title with \» and trailing \\» }");
    let datom = text.protosize().unwrap().datom_form(vec![]).unwrap();
    let mut limits = budget();
    let (head, body) = datom.variant(&mut limits, "Presentation").unwrap();
    assert_eq!(head, "Presentation");
    assert_eq!(body.compose::<Presentation>(&mut limits).unwrap(), value);
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Presentation, rkyv::rancor::Error>(&archive).unwrap(),
        value
    );
    assert!(
        Potential::<Presentation>::from("{ title extra }")
            .actualize(&mut budget())
            .is_err()
    );
}
