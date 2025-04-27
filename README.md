# JLPT Cli
Simple Command Line Utility for generating quizes based on different JLPT levels Kanji & Grammar.

The CLI will generate a prompt that can be injected into any LLM. This prompt is configured to return a JSON object that the CLI can understand and parse.


## Example Usage

### Generating a Prompt
To generate a prompt which you can then give to an LLM
```bash
$ jlpt-cli quiz --level n5

"I am teaching a university class of Japanese Language.\n\nGenerate a quiz consisting of 10 questions.\nThe quiz is based off of the JLPT (Japanese Language Proficiency Test)\nand the level we are quizzing on is level N5.\n\nYou MUST use only kanji & vocabulary made of the following kanji:\n日,一,人,年,大,十,二,本,中,出,三,見,月,生,五,上,四,金,九,入,学,円,子,八,六,下,気,小,七,山,女,百,先,名,川,千,水,国,長,男,時,行,分,後,前,間,東,今,高,校,土,外,木,来,車,話,北,午,書,半,白,西,電,天,語,聞,食,何,南,火,右,万,左,休,毎,母,読,友,父,雨\n\nYou MUST only use the following grammar clauses:\nいちばん,が,か,がある,がいる,から,くらい / ぐらい,けど / けれども,する,だ / です,たい,だけ,たことがある,だろう,つもり,で,ている,てから,てください,でしょう,てはいけない,てもいい,と,な,ないでください,なる,に / へ,に,にいく,にする,の,のが下手,のが上手,のが好き,ので,は,ほうがいい,前に,まだ,まで,も,もう,や,より○○のほうが○○\n\nThese are the only subjects that have been taught in the class, and any\nvocabulary made up of kanji that was not in the list above will not be able to be read\nby the students and therefor they will not be able to complete this quiz, which is unacceptable.\n\nThe test should be made of the following question types:\n  - Sentence Completion -> The student will be given a sentence where a grammar piece is missing. The student will then need to choose between 4 different grammar choices to create a comprehensible and grammatically correct sentence.\n  - Reading Comprehension -> The student will be given a small text piece using the kanji & grammar stated above. There will then be several questions asking about the content of the text piece, to check if the student understood what he read.\n\nEach answer will come with a detailed response on why it is the correct answer and should be chosen instead of the rest.\n\nYou will respond in **JSON** format only, without any supporting text.\nThe ENTIRE response will be in a single line (that means absolutely NO new lines), and any quotes will be escaped with backslashes.\nThe response format should be like so:\n\nSentence Completion response format:\n[{\"question_type\": \"\", \"questions\": [{\"question\": \"\", choices: [\"\", \"\", ...] \"answer\": \"\", \"answer_explanation\": \"\"}, ...]}]\n\nReading Comprehension response format:\n[{\"question_type\": \"\", text_piece: \"\", \"questions\": [{\"question\": \"\", choices: [\"\", \"\", ...] \"answer\": \"\", \"answer_explanation\": \"\"}, ...]}]\n"
```


### Displaying Quiz
After injecting the generated prompt into an LLM, you can display the results nicely with the `--prompt_result` flag:
```bash
$ jlpt-cli quiz --level n5 --prompt_result '[{"question_type": "SentenceCompletion", "questions": [{"question": "今日の天気は白いくも＿＿あります。", "choices": ["が", "に", "で", "を"], "answer": "が", "answer_explanation": "'が' is used to describe a characteristic of the subject. 'くもがある' means 'there are clouds'."}, {"question": "父は川＿＿魚をつりました。", "choices": ["に", "で", "へ", "を"], "answer": "で", "answer_explanation": "'で' indicates the place where an action occurs. Here, the action (fishing) happens at the river."}, {"question": "学校へ行く＿＿、本を買いました。", "choices": ["てから", "のが好き", "たい", "だけ"], "answer": "てから", "answer_explanation": "'てから' shows that one action happens after another. 'After going to school, I bought a book.'"}, {"question": "この山は日本で＿＿高いです。", "choices": ["いちばん", "だけ", "より", "だろう"], "answer": "いちばん", "answer_explanation": "'いちばん' means 'the most'. Here it is used to express the highest mountain in Japan."}, {"question": "友だちと話すの＿＿上手です。", "choices": ["が", "で", "に", "を"], "answer": "が", "answer_explanation": "'のが上手です' requires 'が' to link the activity (talking with friends) to the adjective (skillful)."}]}, {"question_type": "ReadingComprehension", "text_piece": "毎年、父と母といっしょに山へ行きます。山ではたくさんの木を見たり、川で水を飲んだりします。今年は三月に行くつもりです。", "questions": [{"question": "だれと山へ行きますか。", "choices": ["一人で", "友だちと", "父と母と", "先生と"], "answer": "父と母と", "answer_explanation": "The text says '父と母といっしょに', meaning 'together with father and mother'."}, {"question": "山で何をしますか。", "choices": ["水を飲みます", "食べます", "車を見ます", "学校へ行きます"], "answer": "水を飲みます", "answer_explanation": "The text mentions '川で水を飲んだりします', meaning 'drink water at the river'."}, {"question": "今年はいつ行きますか。", "choices": ["五月", "三月", "六月", "九月"], "answer": "三月", "answer_explanation": "The text says '今年は三月に行くつもりです', meaning 'this year, planning to go in March'."}]}]'

───────────────────────────────
Sentence Completion
───────────────────────────────
Question: 今日の天気は白いくも＿＿あります。
が, に, で, を

Answer: が

Question: 父は川＿＿魚をつりました。
に, で, へ, を

Answer: で

Question: 学校へ行く＿＿、本を買いました。
てから, のが好き, たい, だけ

Answer: てから

Question: この山は日本で＿＿高いです。
いちばん, だけ, より, だろう

Answer: いちばん

Question: 友だちと話すの＿＿上手です。
が, で, に, を

Answer: が

───────────────────────────────
Reading Comprehension
毎年、父と母といっしょに山へ行きます。山ではたくさんの木を見たり、川で水を飲んだりします。今年は三月に行くつもりです。
───────────────────────────────
Question: だれと山へ行きますか。
一人で, 友だちと, 父と母と, 先生と

Answer: 父と母と

Question: 山で何をしますか。
水を飲みます, 食べます, 車を見ます, 学校へ行きます

Answer: 水を飲みます

Question: 今年はいつ行きますか。
五月, 三月, 六月, 九月

Answer: 三月
```


## Configuring
All JLPT levels, Kanji, Grammar & Prompts can be configured through the `config.yaml` file.

The YAML follows the structure found in `src/config.rs`, where the top level struct is `Configuration`.