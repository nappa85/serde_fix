use std::{borrow::Cow, convert::TryFrom};

use serde::{Serialize, Deserialize};

use crate::entities::{LocalMktDate, MonthYear, UTCTimestamp, data_field};

use super::time_unit::TimeUnit;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvntGrps {
    // #[serde(rename = "864")]
    len: usize,
    inner: Vec<EvntGrp>,
}

impl<'de> Deserialize<'de> for EvntGrps {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<EvntGrps, D::Error> {
        let temp = <Cow<'_, str> as Deserialize>::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        let mut groups = EvntGrps::default();
        let mut actual = EvntGrp::default();
        let iterator = temp.split('\u{1}').map(|a| match a.find('=') {
            Some(i) => (&a[0..i], &a[(i + 1)..]),
            None => ("", &a[0..]),
        });
        for (code, value) in iterator {
            match code {
                "865" => {
                    if actual.event_type.is_some()
                    || actual.event_date.is_some()
                    || actual.event_time.is_some()
                    || actual.event_px.is_some()
                    || actual.event_text.is_some()
                    || actual.event_time_unit.is_some()
                    || actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_type = Some(crate::from_str(value).map_err(serde::de::Error::custom)?);
                },
                "866" => {
                    if actual.event_date.is_some()
                    || actual.event_time.is_some()
                    || actual.event_px.is_some()
                    || actual.event_text.is_some()
                    || actual.event_time_unit.is_some()
                    || actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_date = Some(LocalMktDate::try_from(value).map_err(serde::de::Error::custom)?);
                },
                "1145" => {
                    if actual.event_time.is_some()
                    || actual.event_px.is_some()
                    || actual.event_text.is_some()
                    || actual.event_time_unit.is_some()
                    || actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_time = Some(UTCTimestamp::try_from(value).map_err(serde::de::Error::custom)?);
                },
                "867" => {
                    if actual.event_px.is_some()
                    || actual.event_text.is_some()
                    || actual.event_time_unit.is_some()
                    || actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_px = Some(value.parse().map_err(serde::de::Error::custom)?);
                },
                "868" => {
                    if actual.event_text.is_some()
                    || actual.event_time_unit.is_some()
                    || actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_text = Some(value.to_string());
                },
                "1827" => {
                    if actual.event_time_unit.is_some()
                    || actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_time_unit = Some(crate::from_str(value).map_err(serde::de::Error::custom)?);
                },
                "1826" => {
                    if actual.event_time_period.is_some()
                    || actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_time_period = Some(value.parse().map_err(serde::de::Error::custom)?);
                },
                "2340" => {
                    if actual.event_month_year.is_some()
                    || actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.event_month_year = Some(value.to_string());
                },
                "1578" | "1579" => {
                    if actual.encoded_event_text.is_some() {
                        groups.inner.push(actual);
                        actual = EvntGrp::default();
                    }
                    actual.encoded_event_text = Some(crate::from_str(value).map_err(serde::de::Error::custom)?);
                },
                // 864
                _ => {
                    groups.len = value.parse().map_err(serde::de::Error::custom)?;
                },
            }
        }
        if actual.event_type.is_some()
        || actual.event_date.is_some()
        || actual.event_time.is_some()
        || actual.event_px.is_some()
        || actual.event_text.is_some()
        || actual.event_time_unit.is_some()
        || actual.event_time_period.is_some()
        || actual.event_month_year.is_some()
        || actual.encoded_event_text.is_some() {
            groups.inner.push(actual);
        }
        Ok(groups)
    }
}

impl Serialize for EvntGrps {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut temp = vec![self.len.to_string()];
        for hop in &self.inner {
            if let Some(s) = &hop.event_type {
                temp.push(format!("865={}", crate::to_string(s).map_err(serde::ser::Error::custom)?));
            }
            if let Some(s) = &hop.event_date {
                temp.push(format!("866={}", s.to_string()));
            }
            if let Some(s) = &hop.event_time {
                temp.push(format!("1145={}", s.to_string()));
            }
            if let Some(s) = &hop.event_px {
                temp.push(format!("867={}", s));
            }
            if let Some(s) = &hop.event_text {
                temp.push(format!("868={}", s));
            }
            if let Some(s) = &hop.event_time_unit {
                temp.push(format!("1827={}", crate::to_string(s).map_err(serde::ser::Error::custom)?));
            }
            if let Some(s) = &hop.event_time_period {
                temp.push(format!("1826={}", s));
            }
            if let Some(s) = &hop.event_month_year {
                temp.push(format!("2340={}", s));
            }
            if let Some(s) = &hop.encoded_event_text {
                temp.push(format!("1578={}", crate::to_string(s).map_err(serde::ser::Error::custom)?));
            }
        }
        temp.join("\u{1}").serialize(serializer)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EvntGrp {
    /// Required if NoEvents(864) > 0.
    // #[serde(rename = "865")]
    pub event_type: Option<EventType>,
    /// Conditionally required when EventTime(1145) is specified.
    // #[serde(rename = "866")]
    pub event_date: Option<LocalMktDate>,
    // #[serde(rename = "1145")]
    pub event_time: Option<UTCTimestamp>,
    // #[serde(rename = "867")]
    pub event_px: Option<f64>,
    // #[serde(rename = "868")]
    pub event_text: Option<String>,
    /// Conditionally required when EventTimePeriod(1826) is specified.
    // #[serde(rename = "1827")]
    pub event_time_unit: Option<TimeUnit>,
    /// Conditionally required when EventTimeUnit(1827) is specified.
    // #[serde(rename = "1826")]
    pub event_time_period: Option<i32>,
    // #[serde(rename = "2340")]
    pub event_month_year: Option<MonthYear>,
    /// Must be set if EncodedEventText(1579) field is specified and must immediately precede it.
    /// Encoded (non-ASCII characters) representation of the EventText(868) field in the encoded format specified via the MessageEncoding(347) field.
    // #[serde(rename = "1578")]
    // #[serde(alias = "1579")]
    pub encoded_event_text: Option<EncodedEventText>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EventType {
    /// Put
    #[serde(rename = "1")]
    Put,
    /// Call
    #[serde(rename = "2")]
    Call,
    /// Tender
    #[serde(rename = "3")]
    Tender,
    /// Sinking Fund Call
    #[serde(rename = "4")]
    SinkingFundCall,
    /// Activation
    #[serde(rename = "5")]
    Activation,
    /// Inactiviation
    #[serde(rename = "6")]
    Inactiviation,
    /// Last Eligible Trade Date
    #[serde(rename = "7")]
    LastEligibleTradeDate,
    /// Swap start date
    #[serde(rename = "8")]
    SwapStartDate,
    /// Swap end date
    #[serde(rename = "9")]
    SwapEndDate,
    /// Swap roll date
    #[serde(rename = "10")]
    SwapRollDate,
    /// Swap next start date
    #[serde(rename = "11")]
    SwapNextStartDate,
    /// Swap next roll date
    #[serde(rename = "12")]
    SwapNextRollDate,
    /// First delivery date
    #[serde(rename = "13")]
    FirstDeliveryDate,
    /// Last delivery date
    #[serde(rename = "14")]
    LastDeliveryDate,
    /// Initiatl inventory due date
    #[serde(rename = "15")]
    InitiatlInventoryDueDate,
    /// Final inventory due date
    #[serde(rename = "16")]
    FinalInventoryDueDate,
    /// First intent date
    #[serde(rename = "17")]
    FirstIntentDate,
    /// Last intent date
    #[serde(rename = "18")]
    LastIntentDate,
    /// Position removal date
    #[serde(rename = "19")]
    PositionRemovalDate,
    /// Other
    #[serde(rename = "99")]
    Other,
    /// Minimum notice
    #[serde(rename = "20")]
    MinimumNotice,
    /// Delivery start time
    #[serde(rename = "21")]
    DeliveryStartTime,
    /// Delivery end time
    #[serde(rename = "22")]
    DeliveryEndTime,
    /// First notice date (The first day that a notice of intent to deliver a commodity can be made by a clearing house to a buyer in fulfillment of a given month's futures contract)
    #[serde(rename = "23")]
    FirstNoticeDate,
    /// Last notice date (The last day on which a clearing house may inform an investor that a seller intends to make delivery of a commodity that the investor previously bought in a futures contract. The date is governed by the rules of different exchanges and clearing houses, but may also be stated in the futures contract itself)
    #[serde(rename = "24")]
    LastNoticeDate,
    /// First exercise date
    #[serde(rename = "25")]
    FirstExerciseDate,
    /// Redemption date
    #[serde(rename = "26")]
    RedemptionDate,
    /// Trade continuation effective date
    #[serde(rename = "27")]
    TradeContinuationEffectiveDate,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncodedEventText {
    // #[serde(rename = "1578")]
    len: usize,
    // #[serde(rename = "1579")]
    data: String,
}

impl data_field::DataField for EncodedEventText {
    fn get_len(&self) -> &usize {
        &self.len
    }
    fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    fn get_data(&self) -> &str {
        &self.data
    }
    fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

impl<'de> Deserialize<'de> for EncodedEventText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        data_field::deserialize(deserializer, "1579")
    }
}

impl Serialize for EncodedEventText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        data_field::serialize(self, serializer, "1579")
    }
}
