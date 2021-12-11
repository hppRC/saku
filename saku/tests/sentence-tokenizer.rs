use saku::SentenceTokenizer;

#[test]
fn test_tokenize_short() {
    let document = "どうもこんにちは。私の名前は山田です。「どーも。」で囲んでいます。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec![
        "どうもこんにちは。",
        "私の名前は山田です。",
        "「どーも。」で囲んでいます。",
    ];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}
#[test]
fn test_tokenize_short_mismatch_parenthesis_char_format() {
    let document = "どうもこんにちは。私の名前は山田です。(どーも。）で囲んでいます。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec![
        "どうもこんにちは。",
        "私の名前は山田です。",
        "(どーも。",
        "）で囲んでいます。",
    ];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_short_mismatch_parenthesis_char_format_with_custom_pattern() {
    let document = "「どうもこんにちは。私の名前は山田です。」(どーも。）で囲んでいます。";
    const PAT :[[char; 2]; 1]=[['(', '）']];
    let tokenizer = SentenceTokenizer::new(None, Some(&PAT));

    let expected = vec![
        "「どうもこんにちは。",
        "私の名前は山田です。",
        "」(どーも。）で囲んでいます。",
    ];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn short_nested_parenthesis() {
    let document = "「どうもこんにちは。私の名前は山田です。(どーも。)で囲んでいます。」";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec!["「どうもこんにちは。私の名前は山田です。(どーも。)で囲んでいます。」"];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn short_missing_right_parenthesis() {
    let document = "(どうもこんにちは。「どーも。で囲んでいます。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec!["(どうもこんにちは。", "「どーも。で囲んでいます。"];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn short_missing_left_parenthesis() {
    let document = "どうもこんにちは。)どーも。」で囲んでいます。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec!["どうもこんにちは。", ")どーも。", "」で囲んでいます。"];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn short_mismatched_parenthesis() {
    let document = "「どーも。)で囲んでいます。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec!["「どーも。)で囲んでいます。"];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn short_mismatched_nested_parenthesis() {
    let document = "「どーも。)で囲んでいます。(どーも。」で囲んでいます。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec!["「どーも。)で囲んでいます。(どーも。」で囲んでいます。"];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_medium() {
    let document = r"
吾輩は猫である。名前はまだない。
どこで生れたか頓（とん）と見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。
    ".trim();
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec![
        "吾輩は猫である。",
        "名前はまだない。",
        "どこで生れたか頓（とん）と見当がつかぬ。",
        "何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_long() {
    let document = r"
当初、トミー・ウィソーは「自分を演じられる俳優はジョニー・デップだけだ」と主張して譲らなかったが、最終的にジェームズ・フランコが自身を演じることを認めた。
フランコはその顛末について
「あるとき、トミーは私が自分を演じることを許可したのです。駆け出しの頃に私がジェームズ・ディーンを演じたから、
トミーは許可を出したのでしょう。皆さんお分かりのように、トミーはディーンには似てません。
私の目には、トミーがマジックペンで髪を染めた吸血鬼に見えます。しかし、トミーは自分をジェームズ・ディーンだと思っているのです。」と語っている。
    ".trim();
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec![
        "当初、トミー・ウィソーは「自分を演じられる俳優はジョニー・デップだけだ」と主張して譲らなかったが、最終的にジェームズ・フランコが自身を演じることを認めた。",
        "フランコはその顛末について「あるとき、トミーは私が自分を演じることを許可したのです。駆け出しの頃に私がジェームズ・ディーンを演じたから、トミーは許可を出したのでしょう。皆さんお分かりのように、トミーはディーンには似てません。私の目には、トミーがマジックペンで髪を染めた吸血鬼に見えます。しかし、トミーは自分をジェームズ・ディーンだと思っているのです。」と語っている。"];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_complex() {
    let document = "吾輩は「猫である。名前はまだない。どこで生れたか」頓（とん）と見当がつかぬ。何でも（薄暗い。。。）じめじめした所で『ニャーニャー。』泣いていた。事だけは記憶している。";
    let tokenizer = SentenceTokenizer::new(None, None);

    let expected = vec![
        "吾輩は「猫である。名前はまだない。どこで生れたか」頓（とん）と見当がつかぬ。",
        "何でも（薄暗い。。。）じめじめした所で『ニャーニャー。』泣いていた。",
        "事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize(document, false);
    assert_eq!(expected, actual);
}
