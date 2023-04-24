use serde::{Deserialize, Serialize};

/// Locale Filter
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Locale {
    EnUs,
    PtBr,
    EsEs,
    CaEs,
    DeDe,
    ItIt,
    FrFr,
    SvSe,
    IdId,
    PlPl,
    JaJp,
    ZhTw,
    ZhCn,
    KoKr,
    ThTh,
    NlNl,
    HuHu,
    ViVn,
    CsCz,
    DaDk,
    FiFi,
    UkUa,
    ElGr,
    RoRo,
    NbNo,
    SkSk,
    TrTr,
    RuRr,
}
