use saku::SentenceTokenizer;

#[test]
fn test_tokenize_short() {
    let document = "どうもこんにちは。私の名前は山田です。「どーも。」で囲んでいます。";
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "どうもこんにちは。",
        "私の名前は山田です。",
        "「どーも。」で囲んでいます。",
    ];
    let actual = tokenizer.tokenize(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_medium() {
    let document = r"
吾輩は猫である。名前はまだない。
どこで生れたか頓（とん）と見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。
    ".trim();
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は猫である。",
        "名前はまだない。",
        "どこで生れたか頓（とん）と見当がつかぬ。",
        "何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize(document);
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
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "当初、トミー・ウィソーは「自分を演じられる俳優はジョニー・デップだけだ」と主張して譲らなかったが、最終的にジェームズ・フランコが自身を演じることを認めた。",
        "フランコはその顛末について「あるとき、トミーは私が自分を演じることを許可したのです。駆け出しの頃に私がジェームズ・ディーンを演じたから、トミーは許可を出したのでしょう。皆さんお分かりのように、トミーはディーンには似てません。私の目には、トミーがマジックペンで髪を染めた吸血鬼に見えます。しかし、トミーは自分をジェームズ・ディーンだと思っているのです。」と語っている。"];
    let actual = tokenizer.tokenize(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_complex() {
    let document = "吾輩は「猫である。名前はまだない。どこで生れたか」頓（とん）と見当がつかぬ。何でも（薄暗い。。。）じめじめした所で『ニャーニャー。』泣いていた。事だけは記憶している。";
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は「猫である。名前はまだない。どこで生れたか」頓（とん）と見当がつかぬ。",
        "何でも（薄暗い。。。）じめじめした所で『ニャーニャー。』泣いていた。",
        "事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_complex_doubly_nested() {
    let document = "吾輩は「猫である。『『名前はまだない。』』どこで生れたか」頓（とん）と見当がつかぬ。何でも（（薄暗い。。。）じめじめした。）所で『ニャーニャー。』泣いていた。事だけは記憶している。";
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は「猫である。『『名前はまだない。』』どこで生れたか」頓（とん）と見当がつかぬ。",
        "何でも（（薄暗い。。。）じめじめした。）所で『ニャーニャー。』泣いていた。",
        "事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_with_sentence_fragment() {
    let document = "吾輩は猫である。名前はまだない。どこで生れたか頓（とん）と見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー";
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は猫である。",
        "名前はまだない。",
        "どこで生れたか頓（とん）と見当がつかぬ。",
        "何でも薄暗いじめじめした所でニャーニャー",
    ];
    let actual = tokenizer.tokenize(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_raw_medium() {
    let document = r"
吾輩は猫である。名前はまだない。
どこで生れたか頓（とん）と見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。
    ".trim();
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は猫である。",
        "名前はまだない。",
        "どこで生れたか頓（とん）と見当がつかぬ。",
        "何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize_raw(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_raw_complex() {
    let document = r"
吾輩は猫である。名前は
まだない。
どこで生れたか頓（とん）と
「見当がつかぬ。何でも薄暗いじめじめした所。でニャーニャー泣いていた」事だけは記憶している。
    "
    .trim();
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は猫である。",
        "名前は",
        "まだない。",
        "どこで生れたか頓（とん）と",
        "「見当がつかぬ。何でも薄暗いじめじめした所。でニャーニャー泣いていた」事だけは記憶している。",
    ];
    let actual = tokenizer.tokenize_raw(document);
    assert_eq!(expected, actual);
}

#[test]
fn test_tokenize_raw_with_sentence_fragment() {
    let document = "吾輩は猫である。名前はまだない。どこで生れたか
頓（とん）と見当がつかぬ。何でも薄暗いじめじめした所でニャーニャー";
    let tokenizer = SentenceTokenizer::default();

    let expected = vec![
        "吾輩は猫である。",
        "名前はまだない。",
        "どこで生れたか",
        "頓（とん）と見当がつかぬ。",
        "何でも薄暗いじめじめした所でニャーニャー",
    ];
    let actual = tokenizer.tokenize_raw(document);
    assert_eq!(expected, actual);
}
