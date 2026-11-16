use signal_flow::{
    Address, Aspect, Input, Model, Output, RequestId, SendRequest, SendSeveralRequest,
    SendableMessage, SendableRequest, Topic,
};

#[test]
fn several_messages_archive_in_written_order_for_one_recipient() {
    let request = SendSeveralRequest {
        address: Address {
            aspect: Aspect::Mind,
            topic: Topic("core".to_string()),
            model: Model::Sol,
        },
        sendable_message_vector: vec![
            SendableMessage::Soft(SendableRequest::Order("first".to_string())),
            SendableMessage::HardAbrupt(SendableRequest::Question("second".to_string())),
            SendableMessage::MiddleAbrupt(SendableRequest::Notice("third".to_string())),
        ],
    };
    let query = Input::SendSeveral(request.clone());
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&query).unwrap();
    let restored = rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap();
    assert_eq!(restored, query);
    let Input::SendSeveral(restored) = restored else {
        panic!("SendSeveral");
    };
    assert_eq!(restored.address, request.address);
    assert_eq!(
        restored.sendable_message_vector,
        request.sendable_message_vector
    );

    // The existing single-message request still carries exactly one message.
    let single = Input::Send(SendRequest {
        address: request.address,
        sendable_message: request.sendable_message_vector[0].clone(),
    });
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&single).unwrap();
    assert_eq!(
        rkyv::from_bytes::<Input, rkyv::rancor::Error>(&bytes).unwrap(),
        single
    );
}

#[test]
fn several_receipt_ids_archive_in_written_order_including_an_empty_receipt() {
    for ids in [
        vec![
            RequestId("request-first".to_string()),
            RequestId("request-second".to_string()),
            RequestId("request-third".to_string()),
        ],
        vec![],
    ] {
        let reply = Output::SentSeveral(ids);
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).unwrap();
        assert_eq!(
            rkyv::from_bytes::<Output, rkyv::rancor::Error>(&bytes).unwrap(),
            reply
        );
    }
}

#[cfg(feature = "datom")]
#[test]
fn several_datoms_preserve_message_and_id_order_and_allow_empty_vectors() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};

    for text in [
        "SendSeveral.{ { Mind core Sol } [ Soft.Order.first HardAbrupt.Question.second MiddleAbrupt.Notice.third ] }",
        "SendSeveral.{ { Mind core Sol } [] }",
        "SentSeveral.[ request-first request-second request-third ]",
        "SentSeveral.[]",
    ] {
        let mut budget = Budget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096).unwrap(),
            depth: 0,
            maximum_depth: 1024,
        };
        if text.starts_with("SendSeveral.") {
            let query = Potential::<Input>::from(text)
                .actualize(&mut budget)
                .unwrap();
            assert_eq!(query.datomize(vec![]).protosize().compact(), text);
        } else {
            let reply = Potential::<Output>::from(text)
                .actualize(&mut budget)
                .unwrap();
            assert_eq!(reply.datomize(vec![]).protosize().compact(), text);
        }
    }
}
