use kafka_enum::topics;

#[topics]
pub enum Hello {}

#[test]
fn it_works() {
    let i = Hello::Glossary;
    let j = Hello::ExampleTopic;

    assert_eq!(i.as_str(), "glossary");
    assert_eq!(j.as_str(), "example_topic_str")
}
