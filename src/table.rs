#[derive(Debug)]
pub struct EraItem {
    pub name: &'static str,
    pub ruby: &'static str,
    pub ruby_initial: &'static str,
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

pub const ERAS: [EraItem; 242] = [
    EraItem {
        name: "白雉",
        ruby: "はくち",
        ruby_initial: "H",
        year: 650,
        month: 2,
        day: 15,
    },
    EraItem {
        name: "朱鳥",
        ruby: "しゅちょう（すちょう）",
        ruby_initial: "S",
        year: 686,
        month: 7,
        day: 20,
    },
    EraItem {
        name: "大宝",
        ruby: "たいほう（だいほう）",
        ruby_initial: "T",
        year: 701,
        month: 3,
        day: 21,
    },
    EraItem {
        name: "慶雲",
        ruby: "けいうん（きょううん）",
        ruby_initial: "K",
        year: 704,
        month: 5,
        day: 10,
    },
    EraItem {
        name: "和銅",
        ruby: "わどう",
        ruby_initial: "W",
        year: 708,
        month: 1,
        day: 11,
    },
    EraItem {
        name: "養老",
        ruby: "ようろう",
        ruby_initial: "Y",
        year: 717,
        month: 11,
        day: 17,
    },
    EraItem {
        name: "神亀",
        ruby: "じんき",
        ruby_initial: "J",
        year: 724,
        month: 2,
        day: 4,
    },
    EraItem {
        name: "天平",
        ruby: "てんぴょう（てんびょう）",
        ruby_initial: "T",
        year: 729,
        month: 8,
        day: 5,
    },
    EraItem {
        name: "天平感宝",
        ruby: "てんぴょうかんぽう",
        ruby_initial: "T",
        year: 749,
        month: 4,
        day: 14,
    },
    EraItem {
        name: "天平勝宝",
        ruby: "てんぴょうしょうほう",
        ruby_initial: "T",
        year: 749,
        month: 7,
        day: 2,
    },
    EraItem {
        name: "天平宝字",
        ruby: "てんぴょうほうじ",
        ruby_initial: "T",
        year: 757,
        month: 8,
        day: 18,
    },
    EraItem {
        name: "天平神護",
        ruby: "てんぴょうしんご",
        ruby_initial: "T",
        year: 765,
        month: 1,
        day: 7,
    },
    EraItem {
        name: "神護景雲",
        ruby: "しんごけいうん",
        ruby_initial: "S",
        year: 767,
        month: 8,
        day: 16,
    },
    EraItem {
        name: "宝亀",
        ruby: "ほうき",
        ruby_initial: "H",
        year: 770,
        month: 10,
        day: 1,
    },
    EraItem {
        name: "天応",
        ruby: "てんおう",
        ruby_initial: "T",
        year: 781,
        month: 1,
        day: 1,
    },
    EraItem {
        name: "延暦",
        ruby: "えんりゃく",
        ruby_initial: "E",
        year: 782,
        month: 8,
        day: 19,
    },
    EraItem {
        name: "弘仁",
        ruby: "こうにん",
        ruby_initial: "K",
        year: 810,
        month: 9,
        day: 19,
    },
    EraItem {
        name: "天長",
        ruby: "てんちょう",
        ruby_initial: "T",
        year: 824,
        month: 1,
        day: 5,
    },
    EraItem {
        name: "承和",
        ruby: "じょうわ（しょうわ）",
        ruby_initial: "J",
        year: 834,
        month: 1,
        day: 3,
    },
    EraItem {
        name: "嘉祥",
        ruby: "かしょう（かじょう）",
        ruby_initial: "K",
        year: 848,
        month: 6,
        day: 13,
    },
    EraItem {
        name: "仁寿",
        ruby: "にんじゅ",
        ruby_initial: "N",
        year: 851,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "斎衡",
        ruby: "さいこう",
        ruby_initial: "S",
        year: 854,
        month: 11,
        day: 30,
    },
    EraItem {
        name: "天安",
        ruby: "てんあん（てんなん）",
        ruby_initial: "T",
        year: 857,
        month: 2,
        day: 21,
    },
    EraItem {
        name: "貞観",
        ruby: "じょうがん",
        ruby_initial: "J",
        year: 859,
        month: 4,
        day: 15,
    },
    EraItem {
        name: "元慶",
        ruby: "がんぎょう（げんけい）",
        ruby_initial: "G",
        year: 877,
        month: 4,
        day: 16,
    },
    EraItem {
        name: "仁和",
        ruby: "にんな（じんな）",
        ruby_initial: "N",
        year: 885,
        month: 2,
        day: 21,
    },
    EraItem {
        name: "寛平",
        ruby: "かんぴょう（かんぺい）",
        ruby_initial: "K",
        year: 889,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "昌泰",
        ruby: "しょうたい",
        ruby_initial: "S",
        year: 898,
        month: 4,
        day: 26,
    },
    EraItem {
        name: "延喜",
        ruby: "えんぎ",
        ruby_initial: "E",
        year: 901,
        month: 7,
        day: 15,
    },
    EraItem {
        name: "延長",
        ruby: "えんちょう",
        ruby_initial: "E",
        year: 923,
        month: 4,
        day: 11,
    },
    EraItem {
        name: "承平",
        ruby: "じょうへい（しょうへい）",
        ruby_initial: "J",
        year: 931,
        month: 4,
        day: 26,
    },
    EraItem {
        name: "天慶",
        ruby: "てんぎょう（てんきょう）",
        ruby_initial: "T",
        year: 938,
        month: 5,
        day: 22,
    },
    EraItem {
        name: "天暦",
        ruby: "てんりゃく（てんれき）",
        ruby_initial: "T",
        year: 947,
        month: 4,
        day: 22,
    },
    EraItem {
        name: "天徳",
        ruby: "てんとく",
        ruby_initial: "T",
        year: 957,
        month: 10,
        day: 27,
    },
    EraItem {
        name: "応和",
        ruby: "おうわ",
        ruby_initial: "O",
        year: 961,
        month: 2,
        day: 16,
    },
    EraItem {
        name: "康保",
        ruby: "こうほう",
        ruby_initial: "K",
        year: 964,
        month: 7,
        day: 10,
    },
    EraItem {
        name: "安和",
        ruby: "あんな（あんわ）",
        ruby_initial: "A",
        year: 968,
        month: 8,
        day: 13,
    },
    EraItem {
        name: "天禄",
        ruby: "てんろく",
        ruby_initial: "T",
        year: 970,
        month: 3,
        day: 25,
    },
    EraItem {
        name: "天延",
        ruby: "てんえん",
        ruby_initial: "T",
        year: 973,
        month: 12,
        day: 20,
    },
    EraItem {
        name: "貞元",
        ruby: "じょうげん（ていげん）",
        ruby_initial: "J",
        year: 976,
        month: 7,
        day: 13,
    },
    EraItem {
        name: "天元",
        ruby: "てんげん",
        ruby_initial: "T",
        year: 978,
        month: 11,
        day: 29,
    },
    EraItem {
        name: "永観",
        ruby: "えいかん",
        ruby_initial: "E",
        year: 983,
        month: 4,
        day: 15,
    },
    EraItem {
        name: "寛和",
        ruby: "かんな（かんわ）",
        ruby_initial: "K",
        year: 985,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "永延",
        ruby: "えいえん（ようえん）",
        ruby_initial: "E",
        year: 987,
        month: 4,
        day: 5,
    },
    EraItem {
        name: "永祚",
        ruby: "えいそ",
        ruby_initial: "E",
        year: 989,
        month: 8,
        day: 8,
    },
    EraItem {
        name: "正暦",
        ruby: "しょうりゃく（じょうりゃく）",
        ruby_initial: "S",
        year: 990,
        month: 11,
        day: 7,
    },
    EraItem {
        name: "長徳",
        ruby: "ちょうとく",
        ruby_initial: "T",
        year: 995,
        month: 2,
        day: 22,
    },
    EraItem {
        name: "長保",
        ruby: "ちょうほう",
        ruby_initial: "T",
        year: 999,
        month: 1,
        day: 13,
    },
    EraItem {
        name: "寛弘",
        ruby: "かんこう",
        ruby_initial: "K",
        year: 1004,
        month: 7,
        day: 20,
    },
    EraItem {
        name: "長和",
        ruby: "ちょうわ",
        ruby_initial: "T",
        year: 1012,
        month: 12,
        day: 25,
    },
    EraItem {
        name: "寛仁",
        ruby: "かんにん",
        ruby_initial: "K",
        year: 1017,
        month: 4,
        day: 23,
    },
    EraItem {
        name: "治安",
        ruby: "じあん（ちあん）",
        ruby_initial: "J",
        year: 1021,
        month: 2,
        day: 2,
    },
    EraItem {
        name: "万寿",
        ruby: "まんじゅ",
        ruby_initial: "M",
        year: 1024,
        month: 7,
        day: 13,
    },
    EraItem {
        name: "長元",
        ruby: "ちょうげん",
        ruby_initial: "T",
        year: 1028,
        month: 7,
        day: 25,
    },
    EraItem {
        name: "長暦",
        ruby: "ちょうりゃく（ちょうれき）",
        ruby_initial: "T",
        year: 1037,
        month: 4,
        day: 21,
    },
    EraItem {
        name: "長久",
        ruby: "ちょうきゅう",
        ruby_initial: "T",
        year: 1040,
        month: 11,
        day: 10,
    },
    EraItem {
        name: "寛徳",
        ruby: "かんとく",
        ruby_initial: "K",
        year: 1044,
        month: 11,
        day: 24,
    },
    EraItem {
        name: "永承",
        ruby: "えいしょう（えいじょう）",
        ruby_initial: "E",
        year: 1046,
        month: 4,
        day: 14,
    },
    EraItem {
        name: "天喜",
        ruby: "てんぎ（てんき）",
        ruby_initial: "T",
        year: 1053,
        month: 1,
        day: 11,
    },
    EraItem {
        name: "康平",
        ruby: "こうへい",
        ruby_initial: "K",
        year: 1058,
        month: 8,
        day: 29,
    },
    EraItem {
        name: "治暦",
        ruby: "じりゃく（ちりゃく）",
        ruby_initial: "J",
        year: 1065,
        month: 8,
        day: 2,
    },
    EraItem {
        name: "延久",
        ruby: "えんきゅう",
        ruby_initial: "E",
        year: 1069,
        month: 4,
        day: 13,
    },
    EraItem {
        name: "承保",
        ruby: "じょうほう（しょうほう）",
        ruby_initial: "J",
        year: 1074,
        month: 8,
        day: 23,
    },
    EraItem {
        name: "承暦",
        ruby: "じょうりゃく（しょうりゃく）",
        ruby_initial: "J",
        year: 1077,
        month: 11,
        day: 17,
    },
    EraItem {
        name: "永保",
        ruby: "えいほう",
        ruby_initial: "E",
        year: 1081,
        month: 2,
        day: 10,
    },
    EraItem {
        name: "応徳",
        ruby: "おうとく",
        ruby_initial: "O",
        year: 1084,
        month: 2,
        day: 7,
    },
    EraItem {
        name: "寛治",
        ruby: "かんじ",
        ruby_initial: "K",
        year: 1087,
        month: 4,
        day: 7,
    },
    EraItem {
        name: "嘉保",
        ruby: "かほう",
        ruby_initial: "K",
        year: 1094,
        month: 12,
        day: 15,
    },
    EraItem {
        name: "永長",
        ruby: "えいちょう（ようちょう）",
        ruby_initial: "E",
        year: 1096,
        month: 12,
        day: 17,
    },
    EraItem {
        name: "承徳",
        ruby: "じょうとく（しょうとく）",
        ruby_initial: "J",
        year: 1097,
        month: 11,
        day: 21,
    },
    EraItem {
        name: "康和",
        ruby: "こうわ",
        ruby_initial: "K",
        year: 1099,
        month: 8,
        day: 28,
    },
    EraItem {
        name: "長治",
        ruby: "ちょうじ",
        ruby_initial: "T",
        year: 1104,
        month: 2,
        day: 10,
    },
    EraItem {
        name: "嘉承",
        ruby: "かじょう（かしょう）",
        ruby_initial: "K",
        year: 1106,
        month: 4,
        day: 9,
    },
    EraItem {
        name: "天仁",
        ruby: "てんにん",
        ruby_initial: "T",
        year: 1108,
        month: 8,
        day: 3,
    },
    EraItem {
        name: "天永",
        ruby: "てんえい",
        ruby_initial: "T",
        year: 1110,
        month: 7,
        day: 13,
    },
    EraItem {
        name: "永久",
        ruby: "えいきゅう",
        ruby_initial: "E",
        year: 1113,
        month: 7,
        day: 13,
    },
    EraItem {
        name: "元永",
        ruby: "げんえい",
        ruby_initial: "G",
        year: 1118,
        month: 4,
        day: 3,
    },
    EraItem {
        name: "保安",
        ruby: "ほうあん",
        ruby_initial: "H",
        year: 1120,
        month: 4,
        day: 10,
    },
    EraItem {
        name: "天治",
        ruby: "てんじ",
        ruby_initial: "T",
        year: 1124,
        month: 4,
        day: 3,
    },
    EraItem {
        name: "大治",
        ruby: "だいじ（たいじ）",
        ruby_initial: "D",
        year: 1126,
        month: 1,
        day: 22,
    },
    EraItem {
        name: "天承",
        ruby: "てんしょう（てんじょう）",
        ruby_initial: "T",
        year: 1131,
        month: 1,
        day: 29,
    },
    EraItem {
        name: "長承",
        ruby: "ちょうしょう（ちょうじょう）",
        ruby_initial: "T",
        year: 1132,
        month: 8,
        day: 11,
    },
    EraItem {
        name: "保延",
        ruby: "ほうえん",
        ruby_initial: "H",
        year: 1135,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "永治",
        ruby: "えいじ",
        ruby_initial: "E",
        year: 1141,
        month: 7,
        day: 10,
    },
    EraItem {
        name: "康治",
        ruby: "こうじ",
        ruby_initial: "K",
        year: 1142,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "天養",
        ruby: "てんよう",
        ruby_initial: "T",
        year: 1144,
        month: 2,
        day: 23,
    },
    EraItem {
        name: "久安",
        ruby: "きゅうあん",
        ruby_initial: "K",
        year: 1145,
        month: 7,
        day: 22,
    },
    EraItem {
        name: "仁平",
        ruby: "にんぺい（にんびょう）",
        ruby_initial: "N",
        year: 1151,
        month: 1,
        day: 26,
    },
    EraItem {
        name: "久寿",
        ruby: "きゅうじゅ",
        ruby_initial: "K",
        year: 1154,
        month: 10,
        day: 28,
    },
    EraItem {
        name: "保元",
        ruby: "ほうげん",
        ruby_initial: "H",
        year: 1156,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "平治",
        ruby: "へいじ（びょうじ）",
        ruby_initial: "H",
        year: 1159,
        month: 4,
        day: 20,
    },
    EraItem {
        name: "永暦",
        ruby: "えいりゃく（ようりゃく）",
        ruby_initial: "E",
        year: 1160,
        month: 1,
        day: 10,
    },
    EraItem {
        name: "応保",
        ruby: "おうほう",
        ruby_initial: "O",
        year: 1161,
        month: 9,
        day: 4,
    },
    EraItem {
        name: "長寛",
        ruby: "ちょうかん",
        ruby_initial: "T",
        year: 1163,
        month: 3,
        day: 29,
    },
    EraItem {
        name: "永万",
        ruby: "えいまん（ようまん）",
        ruby_initial: "E",
        year: 1165,
        month: 6,
        day: 5,
    },
    EraItem {
        name: "仁安",
        ruby: "にんあん（にんなん）",
        ruby_initial: "N",
        year: 1166,
        month: 8,
        day: 27,
    },
    EraItem {
        name: "嘉応",
        ruby: "かおう",
        ruby_initial: "K",
        year: 1169,
        month: 4,
        day: 8,
    },
    EraItem {
        name: "承安",
        ruby: "じょうあん（しょうあん）",
        ruby_initial: "J",
        year: 1171,
        month: 4,
        day: 21,
    },
    EraItem {
        name: "安元",
        ruby: "あんげん",
        ruby_initial: "A",
        year: 1175,
        month: 7,
        day: 28,
    },
    EraItem {
        name: "治承",
        ruby: "じしょう（じじょう）",
        ruby_initial: "J",
        year: 1177,
        month: 8,
        day: 4,
    },
    EraItem {
        name: "養和",
        ruby: "ようわ",
        ruby_initial: "Y",
        year: 1181,
        month: 7,
        day: 14,
    },
    EraItem {
        name: "寿永",
        ruby: "じゅえい",
        ruby_initial: "J",
        year: 1182,
        month: 5,
        day: 27,
    },
    EraItem {
        name: "元暦",
        ruby: "げんりゃく",
        ruby_initial: "G",
        year: 1184,
        month: 4,
        day: 16,
    },
    EraItem {
        name: "文治",
        ruby: "ぶんじ（もんじ）",
        ruby_initial: "B",
        year: 1185,
        month: 8,
        day: 14,
    },
    EraItem {
        name: "正治",
        ruby: "しょうじ",
        ruby_initial: "S",
        year: 1199,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "建仁",
        ruby: "けんにん",
        ruby_initial: "K",
        year: 1201,
        month: 2,
        day: 13,
    },
    EraItem {
        name: "元久",
        ruby: "げんきゅう",
        ruby_initial: "G",
        year: 1204,
        month: 2,
        day: 20,
    },
    EraItem {
        name: "建永",
        ruby: "けんえい",
        ruby_initial: "K",
        year: 1206,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "承元",
        ruby: "じょうげん（しょうげん）",
        ruby_initial: "J",
        year: 1207,
        month: 10,
        day: 25,
    },
    EraItem {
        name: "建暦",
        ruby: "けんりゃく",
        ruby_initial: "K",
        year: 1211,
        month: 3,
        day: 9,
    },
    EraItem {
        name: "建保",
        ruby: "けんぽう（けんほう）",
        ruby_initial: "K",
        year: 1213,
        month: 12,
        day: 6,
    },
    EraItem {
        name: "承久",
        ruby: "じょうきゅう（しょうきゅう）",
        ruby_initial: "J",
        year: 1219,
        month: 4,
        day: 12,
    },
    EraItem {
        name: "貞応",
        ruby: "じょうおう（ていおう）",
        ruby_initial: "J",
        year: 1222,
        month: 4,
        day: 13,
    },
    EraItem {
        name: "元仁",
        ruby: "げんにん",
        ruby_initial: "G",
        year: 1224,
        month: 11,
        day: 20,
    },
    EraItem {
        name: "嘉禄",
        ruby: "かろく",
        ruby_initial: "K",
        year: 1225,
        month: 4,
        day: 20,
    },
    EraItem {
        name: "安貞",
        ruby: "あんてい",
        ruby_initial: "A",
        year: 1227,
        month: 12,
        day: 10,
    },
    EraItem {
        name: "寛喜",
        ruby: "かんき",
        ruby_initial: "K",
        year: 1229,
        month: 3,
        day: 5,
    },
    EraItem {
        name: "貞永",
        ruby: "じょうえい（ていえい）",
        ruby_initial: "J",
        year: 1232,
        month: 4,
        day: 2,
    },
    EraItem {
        name: "天福",
        ruby: "てんぷく（てんふく）",
        ruby_initial: "T",
        year: 1233,
        month: 4,
        day: 15,
    },
    EraItem {
        name: "文暦",
        ruby: "ぶんりゃく（もんりゃく）",
        ruby_initial: "B",
        year: 1234,
        month: 11,
        day: 5,
    },
    EraItem {
        name: "嘉禎",
        ruby: "かてい",
        ruby_initial: "K",
        year: 1235,
        month: 9,
        day: 19,
    },
    EraItem {
        name: "暦仁",
        ruby: "りゃくにん（れきにん）",
        ruby_initial: "R",
        year: 1238,
        month: 11,
        day: 23,
    },
    EraItem {
        name: "延応",
        ruby: "えんおう（えんのう）",
        ruby_initial: "E",
        year: 1239,
        month: 2,
        day: 7,
    },
    EraItem {
        name: "仁治",
        ruby: "にんじ（にんち）",
        ruby_initial: "N",
        year: 1240,
        month: 7,
        day: 16,
    },
    EraItem {
        name: "寛元",
        ruby: "かんげん",
        ruby_initial: "K",
        year: 1243,
        month: 2,
        day: 26,
    },
    EraItem {
        name: "宝治",
        ruby: "ほうじ",
        ruby_initial: "H",
        year: 1247,
        month: 2,
        day: 28,
    },
    EraItem {
        name: "建長",
        ruby: "けんちょう",
        ruby_initial: "K",
        year: 1249,
        month: 3,
        day: 18,
    },
    EraItem {
        name: "康元",
        ruby: "こうげん",
        ruby_initial: "K",
        year: 1256,
        month: 10,
        day: 5,
    },
    EraItem {
        name: "正嘉",
        ruby: "しょうか",
        ruby_initial: "S",
        year: 1257,
        month: 3,
        day: 14,
    },
    EraItem {
        name: "正元",
        ruby: "しょうげん",
        ruby_initial: "S",
        year: 1259,
        month: 3,
        day: 26,
    },
    EraItem {
        name: "文応",
        ruby: "ぶんおう",
        ruby_initial: "B",
        year: 1260,
        month: 4,
        day: 13,
    },
    EraItem {
        name: "弘長",
        ruby: "こうちょう",
        ruby_initial: "K",
        year: 1261,
        month: 2,
        day: 20,
    },
    EraItem {
        name: "文永",
        ruby: "ぶんえい",
        ruby_initial: "B",
        year: 1264,
        month: 2,
        day: 28,
    },
    EraItem {
        name: "建治",
        ruby: "けんじ",
        ruby_initial: "K",
        year: 1275,
        month: 4,
        day: 25,
    },
    EraItem {
        name: "弘安",
        ruby: "こうあん",
        ruby_initial: "K",
        year: 1278,
        month: 3,
        day: 1,
    },
    EraItem {
        name: "正応",
        ruby: "しょうおう",
        ruby_initial: "S",
        year: 1288,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "永仁",
        ruby: "えいにん",
        ruby_initial: "E",
        year: 1293,
        month: 8,
        day: 5,
    },
    EraItem {
        name: "正安",
        ruby: "しょうあん",
        ruby_initial: "S",
        year: 1299,
        month: 4,
        day: 25,
    },
    EraItem {
        name: "乾元",
        ruby: "けんげん",
        ruby_initial: "K",
        year: 1302,
        month: 11,
        day: 21,
    },
    EraItem {
        name: "嘉元",
        ruby: "かげん",
        ruby_initial: "K",
        year: 1303,
        month: 8,
        day: 5,
    },
    EraItem {
        name: "徳治",
        ruby: "とくじ",
        ruby_initial: "T",
        year: 1306,
        month: 12,
        day: 14,
    },
    EraItem {
        name: "延慶",
        ruby: "えんきょう（えんぎょう）",
        ruby_initial: "E",
        year: 1308,
        month: 10,
        day: 9,
    },
    EraItem {
        name: "応長",
        ruby: "おうちょう",
        ruby_initial: "O",
        year: 1311,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "正和",
        ruby: "しょうわ",
        ruby_initial: "S",
        year: 1312,
        month: 3,
        day: 20,
    },
    EraItem {
        name: "文保",
        ruby: "ぶんぽう（ぶんほう）",
        ruby_initial: "B",
        year: 1317,
        month: 2,
        day: 3,
    },
    EraItem {
        name: "元応",
        ruby: "げんおう（げんのう）",
        ruby_initial: "G",
        year: 1319,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "元亨",
        ruby: "げんこう",
        ruby_initial: "G",
        year: 1321,
        month: 2,
        day: 23,
    },
    EraItem {
        name: "正中",
        ruby: "しょうちゅう",
        ruby_initial: "S",
        year: 1324,
        month: 12,
        day: 9,
    },
    EraItem {
        name: "嘉暦",
        ruby: "かりゃく",
        ruby_initial: "K",
        year: 1326,
        month: 4,
        day: 26,
    },
    EraItem {
        name: "元徳",
        ruby: "げんとく",
        ruby_initial: "G",
        year: 1329,
        month: 8,
        day: 29,
    },
    EraItem {
        name: "元弘",
        ruby: "げんこう",
        ruby_initial: "G",
        year: 1331,
        month: 8,
        day: 9,
    },
    EraItem {
        name: "正慶",
        ruby: "しょうきょう（しょうけい）",
        ruby_initial: "S",
        year: 1332,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "建武",
        ruby: "けんむ（けんぶ）",
        ruby_initial: "K",
        year: 1334,
        month: 1,
        day: 29,
    },
    EraItem {
        name: "延元",
        ruby: "えんげん",
        ruby_initial: "E",
        year: 1336,
        month: 2,
        day: 29,
    },
    EraItem {
        name: "暦応",
        ruby: "りゃくおう（れきおう）",
        ruby_initial: "R",
        year: 1338,
        month: 8,
        day: 28,
    },
    EraItem {
        name: "興国",
        ruby: "こうこく",
        ruby_initial: "K",
        year: 1340,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "康永",
        ruby: "こうえい",
        ruby_initial: "K",
        year: 1342,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "貞和",
        ruby: "じょうわ（ていわ）",
        ruby_initial: "J",
        year: 1345,
        month: 10,
        day: 21,
    },
    EraItem {
        name: "正平",
        ruby: "しょうへい",
        ruby_initial: "S",
        year: 1346,
        month: 12,
        day: 8,
    },
    EraItem {
        name: "観応",
        ruby: "かんおう（かんのう）",
        ruby_initial: "K",
        year: 1350,
        month: 2,
        day: 27,
    },
    EraItem {
        name: "文和",
        ruby: "ぶんな（ぶんわ）",
        ruby_initial: "B",
        year: 1352,
        month: 9,
        day: 27,
    },
    EraItem {
        name: "延文",
        ruby: "えんぶん",
        ruby_initial: "E",
        year: 1356,
        month: 3,
        day: 28,
    },
    EraItem {
        name: "康安",
        ruby: "こうあん",
        ruby_initial: "K",
        year: 1361,
        month: 3,
        day: 29,
    },
    EraItem {
        name: "貞治",
        ruby: "じょうじ（ていじ）",
        ruby_initial: "J",
        year: 1362,
        month: 9,
        day: 23,
    },
    EraItem {
        name: "応安",
        ruby: "おうあん",
        ruby_initial: "O",
        year: 1368,
        month: 2,
        day: 18,
    },
    EraItem {
        name: "建徳",
        ruby: "けんとく",
        ruby_initial: "K",
        year: 1370,
        month: 7,
        day: 24,
    },
    EraItem {
        name: "文中",
        ruby: "ぶんちゅう",
        ruby_initial: "B",
        year: 1372,
        month: 4,
        day: 1,
    },
    EraItem {
        name: "天授",
        ruby: "てんじゅ",
        ruby_initial: "T",
        year: 1375,
        month: 5,
        day: 27,
    },
    EraItem {
        name: "永和",
        ruby: "えいわ",
        ruby_initial: "E",
        year: 1375,
        month: 2,
        day: 27,
    },
    EraItem {
        name: "康暦",
        ruby: "こうりゃく",
        ruby_initial: "K",
        year: 1379,
        month: 3,
        day: 22,
    },
    EraItem {
        name: "弘和",
        ruby: "こうわ",
        ruby_initial: "K",
        year: 1381,
        month: 2,
        day: 10,
    },
    EraItem {
        name: "永徳",
        ruby: "えいとく",
        ruby_initial: "E",
        year: 1381,
        month: 2,
        day: 24,
    },
    EraItem {
        name: "元中",
        ruby: "げんちゅう",
        ruby_initial: "G",
        year: 1384,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "至徳",
        ruby: "しとく",
        ruby_initial: "S",
        year: 1384,
        month: 2,
        day: 27,
    },
    EraItem {
        name: "嘉慶",
        ruby: "かきょう（かけい）",
        ruby_initial: "K",
        year: 1387,
        month: 8,
        day: 23,
    },
    EraItem {
        name: "康応",
        ruby: "こうおう",
        ruby_initial: "K",
        year: 1389,
        month: 2,
        day: 9,
    },
    EraItem {
        name: "明徳",
        ruby: "めいとく",
        ruby_initial: "M",
        year: 1390,
        month: 3,
        day: 26,
    },
    EraItem {
        name: "応永",
        ruby: "おうえい",
        ruby_initial: "O",
        year: 1394,
        month: 7,
        day: 5,
    },
    EraItem {
        name: "正長",
        ruby: "しょうちょう",
        ruby_initial: "S",
        year: 1428,
        month: 4,
        day: 27,
    },
    EraItem {
        name: "永享",
        ruby: "えいきょう",
        ruby_initial: "E",
        year: 1429,
        month: 9,
        day: 5,
    },
    EraItem {
        name: "嘉吉",
        ruby: "かきつ（かきち）",
        ruby_initial: "K",
        year: 1441,
        month: 2,
        day: 17,
    },
    EraItem {
        name: "文安",
        ruby: "ぶんあん",
        ruby_initial: "B",
        year: 1444,
        month: 2,
        day: 5,
    },
    EraItem {
        name: "宝徳",
        ruby: "ほうとく",
        ruby_initial: "H",
        year: 1449,
        month: 7,
        day: 28,
    },
    EraItem {
        name: "享徳",
        ruby: "きょうとく",
        ruby_initial: "K",
        year: 1452,
        month: 7,
        day: 25,
    },
    EraItem {
        name: "康正",
        ruby: "こうしょう",
        ruby_initial: "K",
        year: 1455,
        month: 7,
        day: 25,
    },
    EraItem {
        name: "長禄",
        ruby: "ちょうろく",
        ruby_initial: "T",
        year: 1457,
        month: 9,
        day: 28,
    },
    EraItem {
        name: "寛正",
        ruby: "かんしょう",
        ruby_initial: "K",
        year: 1460,
        month: 12,
        day: 21,
    },
    EraItem {
        name: "文正",
        ruby: "ぶんしょう（もんしょう）",
        ruby_initial: "B",
        year: 1466,
        month: 2,
        day: 28,
    },
    EraItem {
        name: "応仁",
        ruby: "おうにん",
        ruby_initial: "O",
        year: 1467,
        month: 3,
        day: 5,
    },
    EraItem {
        name: "文明",
        ruby: "ぶんめい",
        ruby_initial: "B",
        year: 1469,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "長享",
        ruby: "ちょうきょう",
        ruby_initial: "T",
        year: 1487,
        month: 7,
        day: 20,
    },
    EraItem {
        name: "延徳",
        ruby: "えんとく",
        ruby_initial: "E",
        year: 1489,
        month: 8,
        day: 21,
    },
    EraItem {
        name: "明応",
        ruby: "めいおう",
        ruby_initial: "M",
        year: 1492,
        month: 7,
        day: 19,
    },
    EraItem {
        name: "文亀",
        ruby: "ぶんき",
        ruby_initial: "B",
        year: 1501,
        month: 3,
        day: 1,
    },
    EraItem {
        name: "永正",
        ruby: "えいしょう",
        ruby_initial: "E",
        year: 1504,
        month: 3,
        day: 1,
    },
    EraItem {
        name: "大永",
        ruby: "だいえい",
        ruby_initial: "D",
        year: 1521,
        month: 8,
        day: 23,
    },
    EraItem {
        name: "享禄",
        ruby: "きょうろく",
        ruby_initial: "K",
        year: 1528,
        month: 8,
        day: 20,
    },
    EraItem {
        name: "天文",
        ruby: "てんぶん",
        ruby_initial: "T",
        year: 1532,
        month: 7,
        day: 29,
    },
    EraItem {
        name: "弘治",
        ruby: "こうじ",
        ruby_initial: "K",
        year: 1555,
        month: 10,
        day: 23,
    },
    EraItem {
        name: "永禄",
        ruby: "えいろく",
        ruby_initial: "E",
        year: 1558,
        month: 2,
        day: 28,
    },
    EraItem {
        name: "元亀",
        ruby: "げんき",
        ruby_initial: "G",
        year: 1570,
        month: 4,
        day: 23,
    },
    EraItem {
        name: "文禄",
        ruby: "ぶんろく",
        ruby_initial: "B",
        year: 1592,
        month: 12,
        day: 8,
    },
    EraItem {
        name: "慶長",
        ruby: "けいちょう（きょうちょう）",
        ruby_initial: "K",
        year: 1596,
        month: 10,
        day: 27,
    },
    EraItem {
        name: "寛永",
        ruby: "かんえい",
        ruby_initial: "K",
        year: 1624,
        month: 3,
        day: 1,
    },
    EraItem {
        name: "正保",
        ruby: "しょうほう",
        ruby_initial: "S",
        year: 1644,
        month: 12,
        day: 16,
    },
    EraItem {
        name: "慶安",
        ruby: "けいあん",
        ruby_initial: "K",
        year: 1648,
        month: 2,
        day: 15,
    },
    EraItem {
        name: "承応",
        ruby: "じょうおう（しょうおう）",
        ruby_initial: "J",
        year: 1652,
        month: 9,
        day: 18,
    },
    EraItem {
        name: "明暦",
        ruby: "めいれき（みょうりゃく）",
        ruby_initial: "M",
        year: 1655,
        month: 4,
        day: 13,
    },
    EraItem {
        name: "万治",
        ruby: "まんじ",
        ruby_initial: "M",
        year: 1658,
        month: 7,
        day: 23,
    },
    EraItem {
        name: "寛文",
        ruby: "かんぶん",
        ruby_initial: "K",
        year: 1661,
        month: 4,
        day: 25,
    },
    EraItem {
        name: "延宝",
        ruby: "えんぽう",
        ruby_initial: "E",
        year: 1673,
        month: 9,
        day: 21,
    },
    EraItem {
        name: "天和",
        ruby: "てんな",
        ruby_initial: "T",
        year: 1681,
        month: 9,
        day: 29,
    },
    EraItem {
        name: "貞享",
        ruby: "じょうきょう",
        ruby_initial: "J",
        year: 1684,
        month: 2,
        day: 21,
    },
    EraItem {
        name: "元禄",
        ruby: "げんろく",
        ruby_initial: "G",
        year: 1688,
        month: 9,
        day: 30,
    },
    EraItem {
        name: "宝永",
        ruby: "ほうえい",
        ruby_initial: "H",
        year: 1704,
        month: 3,
        day: 13,
    },
    EraItem {
        name: "正徳",
        ruby: "しょうとく",
        ruby_initial: "S",
        year: 1711,
        month: 4,
        day: 25,
    },
    EraItem {
        name: "享保",
        ruby: "きょうほう（きょうほ）",
        ruby_initial: "K",
        year: 1716,
        month: 6,
        day: 22,
    },
    EraItem {
        name: "元文",
        ruby: "げんぶん",
        ruby_initial: "G",
        year: 1736,
        month: 4,
        day: 28,
    },
    EraItem {
        name: "寛保",
        ruby: "かんぽう（かんほう）",
        ruby_initial: "K",
        year: 1741,
        month: 2,
        day: 27,
    },
    EraItem {
        name: "延享",
        ruby: "えんきょう",
        ruby_initial: "E",
        year: 1744,
        month: 2,
        day: 21,
    },
    EraItem {
        name: "寛延",
        ruby: "かんえん",
        ruby_initial: "K",
        year: 1748,
        month: 7,
        day: 12,
    },
    EraItem {
        name: "宝暦",
        ruby: "ほうれき（ほうりゃく）",
        ruby_initial: "H",
        year: 1751,
        month: 10,
        day: 27,
    },
    EraItem {
        name: "明和",
        ruby: "めいわ",
        ruby_initial: "M",
        year: 1764,
        month: 6,
        day: 2,
    },
    EraItem {
        name: "安永",
        ruby: "あんえい",
        ruby_initial: "A",
        year: 1772,
        month: 11,
        day: 16,
    },
    EraItem {
        name: "天明",
        ruby: "てんめい",
        ruby_initial: "T",
        year: 1781,
        month: 4,
        day: 2,
    },
    EraItem {
        name: "寛政",
        ruby: "かんせい",
        ruby_initial: "K",
        year: 1789,
        month: 1,
        day: 25,
    },
    EraItem {
        name: "享和",
        ruby: "きょうわ",
        ruby_initial: "K",
        year: 1801,
        month: 2,
        day: 5,
    },
    EraItem {
        name: "文化",
        ruby: "ぶんか",
        ruby_initial: "B",
        year: 1804,
        month: 2,
        day: 11,
    },
    EraItem {
        name: "文政",
        ruby: "ぶんせい",
        ruby_initial: "B",
        year: 1818,
        month: 4,
        day: 22,
    },
    EraItem {
        name: "天保",
        ruby: "てんぽう（てんほう）",
        ruby_initial: "T",
        year: 1830,
        month: 12,
        day: 10,
    },
    EraItem {
        name: "弘化",
        ruby: "こうか",
        ruby_initial: "K",
        year: 1844,
        month: 12,
        day: 2,
    },
    EraItem {
        name: "嘉永",
        ruby: "かえい",
        ruby_initial: "K",
        year: 1848,
        month: 2,
        day: 28,
    },
    EraItem {
        name: "安政",
        ruby: "あんせい",
        ruby_initial: "A",
        year: 1854,
        month: 11,
        day: 27,
    },
    EraItem {
        name: "万延",
        ruby: "まんえん",
        ruby_initial: "M",
        year: 1860,
        month: 3,
        day: 18,
    },
    EraItem {
        name: "文久",
        ruby: "ぶんきゅう",
        ruby_initial: "B",
        year: 1861,
        month: 2,
        day: 19,
    },
    EraItem {
        name: "元治",
        ruby: "げんじ",
        ruby_initial: "G",
        year: 1864,
        month: 2,
        day: 20,
    },
    EraItem {
        name: "慶応",
        ruby: "けいおう",
        ruby_initial: "K",
        year: 1865,
        month: 4,
        day: 7,
    },
    EraItem {
        name: "明治",
        ruby: "めいじ",
        ruby_initial: "M",
        year: 1868,
        month: 9,
        day: 8,
    },
    EraItem {
        name: "大正",
        ruby: "たいしょう",
        ruby_initial: "T",
        year: 1912,
        month: 7,
        day: 30,
    },
    EraItem {
        name: "昭和",
        ruby: "しょうわ",
        ruby_initial: "S",
        year: 1926,
        month: 12,
        day: 25,
    },
    EraItem {
        name: "平成",
        ruby: "へいせい",
        ruby_initial: "H",
        year: 1989,
        month: 1,
        day: 8,
    },
    EraItem {
        name: "令和",
        ruby: "れいわ",
        ruby_initial: "R",
        year: 2019,
        month: 5,
        day: 1,
    },
];
