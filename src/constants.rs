use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::collections::HashMap;

use crate::structs::StaticFormat;

pub const BASE_URL: &str = "https://www.youtube.com/watch?v=";

pub const VALID_QUERY_DOMAINS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "music.youtube.com",
    "gaming.youtube.com",
];

pub const AGE_RESTRICTED_URLS: &[&str] = &[
    "support.google.com/youtube/?p=age_restrictions",
    "youtube.com/t/community_guidelines",
];

pub const AUDIO_ENCODING_RANKS: &[&str] = &["mp4a", "mp3", "vorbis", "aac", "opus", "flac"];
pub const VIDEO_ENCODING_RANKS: &[&str] = &[
    "mp4v",
    "avc1",
    "Sorenson H.283",
    "MPEG-4 Visual",
    "VP8",
    "VP9",
    "H.264",
];

pub(crate) static DEFAULT_HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.101 Safari/537.36".parse().unwrap());

    headers
});

pub(crate) static IPV6_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(([0-9a-f]{1,4}:)(:[0-9a-f]{1,4}){1,6}|([0-9a-f]{1,4}:){1,2}(:[0-9a-f]{1,4}){1,5}|([0-9a-f]{1,4}:){1,3}(:[0-9a-f]{1,4}){1,4}|([0-9a-f]{1,4}:){1,4}(:[0-9a-f]{1,4}){1,3}|([0-9a-f]{1,4}:){1,5}(:[0-9a-f]{1,4}){1,2}|([0-9a-f]{1,4}:){1,6}(:[0-9a-f]{1,4})|([0-9a-f]{1,4}:){1,7}(([0-9a-f]{1,4})|:))/(1[0-1]\d|12[0-8]|\d{1,2})$").unwrap()
});

pub(crate) static PARSE_INT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*((\-|\+)?[0-9]+)\s*").unwrap());

pub static FORMATS: Lazy<HashMap<&str, StaticFormat>> = Lazy::new(|| {
    HashMap::from([
        (
            "5",
            StaticFormat {
                mime_type: r#"video/flv; codecs="Sorenson H.283, mp3""#.to_string().to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(250000),
                audio_bitrate: Some(64),
            },
        ),
        (
            "6",
            StaticFormat {
                mime_type: r#"video/flv; codecs="Sorenson H.263, mp3""#.to_string(),
                quality_label: Some("270p".to_string()),
                bitrate: Some(800000),
                audio_bitrate: Some(64),
            },
        ),
        (
            "13",
            StaticFormat {
                mime_type: r#"video/3gp; codecs="MPEG-4 Visual, aac""#.to_string(),
                quality_label: None,
                bitrate: Some(500000),
                audio_bitrate: None,
            },
        ),
        (
            "17",
            StaticFormat {
                mime_type: r#"video/3gp; codecs="MPEG-4 Visual, aac""#.to_string(),
                quality_label: Some("144p".to_string()),
                bitrate: Some(50000),
                audio_bitrate: Some(24),
            },
        ),
        (
            "18",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: Some(96),
            },
        ),
        (
            "22",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(2000000),
                audio_bitrate: Some(192),
            },
        ),
        (
            "34",
            StaticFormat {
                mime_type: r#"video/flv; codecs="H.264, aac""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "35",
            StaticFormat {
                mime_type: r#"video/flv; codecs="H.264, aac""#.to_string(),
                quality_label: Some("480p".to_string()),
                bitrate: Some(800000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "36",
            StaticFormat {
                mime_type: r#"video/3gp; codecs="MPEG-4 Visual, aac""#.to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(175000),
                audio_bitrate: Some(32),
            },
        ),
        (
            "37",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: Some(3000000),
                audio_bitrate: Some(192),
            },
        ),
        (
            "38",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("3072p".to_string()),
                bitrate: Some(3500000),
                audio_bitrate: Some(192),
            },
        ),
        (
            "43",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP8, vorbis""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "44",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP8, vorbis""#.to_string(),
                quality_label: Some("480p".to_string()),
                bitrate: Some(1000000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "45",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP8, vorbis""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(2000000),
                audio_bitrate: Some(192),
            },
        ),
        (
            "46",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="vp8, vorbis""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: None,
                audio_bitrate: Some(192),
            },
        ),
        (
            "82",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: Some(96),
            },
        ),
        (
            "83",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: Some(96),
            },
        ),
        (
            "84",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(2000000),
                audio_bitrate: Some(192),
            },
        ),
        (
            "85",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264, aac""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: Some(3000000),
                audio_bitrate: Some(192),
            },
        ),
        (
            "91",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("144p".to_string()),
                bitrate: Some(100000),
                audio_bitrate: Some(48),
            },
        ),
        (
            "92",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(150000),
                audio_bitrate: Some(48),
            },
        ),
        (
            "93",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "94",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("480p".to_string()),
                bitrate: Some(800000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "95",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(1500000),
                audio_bitrate: Some(256),
            },
        ),
        (
            "96",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: Some(2500000),
                audio_bitrate: Some(256),
            },
        ),
        (
            "100",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="VP8, vorbis""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: None,
                audio_bitrate: Some(128),
            },
        ),
        (
            "101",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="VP8, vorbis""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: None,
                audio_bitrate: Some(192),
            },
        ),
        (
            "102",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="VP8, vorbis""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: None,
                audio_bitrate: Some(192),
            },
        ),
        (
            "120",
            StaticFormat {
                mime_type: r#"video/flv; codecs="H.264, aac""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(2000000),
                audio_bitrate: Some(128),
            },
        ),
        (
            "127",
            StaticFormat {
                mime_type: r#"audio/ts; codecs="aac""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(96),
            },
        ),
        (
            "128",
            StaticFormat {
                mime_type: r#"audio/ts; codecs="aac""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(96),
            },
        ),
        (
            "132",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(150000),
                audio_bitrate: Some(48),
            },
        ),
        (
            "133",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(200000),
                audio_bitrate: None,
            },
        ),
        (
            "134",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(300000),
                audio_bitrate: None,
            },
        ),
        (
            "135",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("480p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: None,
            },
        ),
        (
            "136",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(1000000),
                audio_bitrate: None,
            },
        ),
        (
            "137",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: Some(2500000),
                audio_bitrate: None,
            },
        ),
        (
            "138",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("4320p".to_string()),
                bitrate: Some(13500000),
                audio_bitrate: None,
            },
        ),
        (
            "139",
            StaticFormat {
                mime_type: r#"audio/mp4; codecs="aac""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(48),
            },
        ),
        (
            "140",
            StaticFormat {
                mime_type: r#"audio/m4a; codecs="aac""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(128),
            },
        ),
        (
            "141",
            StaticFormat {
                mime_type: r#"audio/mp4; codecs="aac""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(256),
            },
        ),
        (
            "151",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(50000),
                audio_bitrate: Some(24),
            },
        ),
        (
            "160",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("144p".to_string()),
                bitrate: Some(100000),
                audio_bitrate: None,
            },
        ),
        (
            "171",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="vorbis""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(128),
            },
        ),
        (
            "172",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="vorbis""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(192),
            },
        ),
        (
            "242",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("240p".to_string()),
                bitrate: Some(100000),
                audio_bitrate: None,
            },
        ),
        (
            "243",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("360p".to_string()),
                bitrate: Some(250000),
                audio_bitrate: None,
            },
        ),
        (
            "244",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("480p".to_string()),
                bitrate: Some(500000),
                audio_bitrate: None,
            },
        ),
        (
            "247",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(700000),
                audio_bitrate: None,
            },
        ),
        (
            "248",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: Some(1500000),
                audio_bitrate: None,
            },
        ),
        (
            "249",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="opus""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(48),
            },
        ),
        (
            "250",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="opus""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(64),
            },
        ),
        (
            "251",
            StaticFormat {
                mime_type: r#"audio/webm; codecs="opus""#.to_string(),
                quality_label: None,
                bitrate: None,
                audio_bitrate: Some(160),
            },
        ),
        (
            "264",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("1440p".to_string()),
                bitrate: Some(4000000),
                audio_bitrate: None,
            },
        ),
        (
            "266",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("2160p".to_string()),
                bitrate: Some(12500000),
                audio_bitrate: None,
            },
        ),
        (
            "271",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("1440p".to_string()),
                bitrate: Some(9000000),
                audio_bitrate: None,
            },
        ),
        (
            "272",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("4320p".to_string()),
                bitrate: Some(20000000),
                audio_bitrate: None,
            },
        ),
        (
            "278",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("144p 30fps".to_string()),
                bitrate: Some(80000),
                audio_bitrate: None,
            },
        ),
        (
            "298",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(3000000),
                audio_bitrate: None,
            },
        ),
        (
            "299",
            StaticFormat {
                mime_type: r#"video/mp4; codecs="H.264""#.to_string(),
                quality_label: Some("1080p".to_string()),
                bitrate: Some(5500000),
                audio_bitrate: None,
            },
        ),
        (
            "300",
            StaticFormat {
                mime_type: r#"video/ts; codecs="H.264, aac""#.to_string(),
                quality_label: Some("720p".to_string()),
                bitrate: Some(1318000),
                audio_bitrate: Some(48),
            },
        ),
        (
            "302",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("720p HFR".to_string()),
                bitrate: Some(2500000),
                audio_bitrate: None,
            },
        ),
        (
            "303",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("1080p HFR".to_string()),
                bitrate: Some(5000000),
                audio_bitrate: None,
            },
        ),
        (
            "308",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("1440p HFR".to_string()),
                bitrate: Some(10000000),
                audio_bitrate: None,
            },
        ),
        (
            "313",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("2160p".to_string()),
                bitrate: Some(13000000),
                audio_bitrate: None,
            },
        ),
        (
            "315",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("2160p HFR".to_string()),
                bitrate: Some(20000000),
                audio_bitrate: None,
            },
        ),
        (
            "330",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("144p HDR, HFR".to_string()),
                bitrate: Some(80000),
                audio_bitrate: None,
            },
        ),
        (
            "331",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("240p HDR, HFR".to_string()),
                bitrate: Some(100000),
                audio_bitrate: None,
            },
        ),
        (
            "332",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("360p HDR, HFR".to_string()),
                bitrate: Some(250000),
                audio_bitrate: None,
            },
        ),
        (
            "333",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("240p HDR, HFR".to_string()),
                bitrate: Some(500000),
                audio_bitrate: None,
            },
        ),
        (
            "334",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("720p HDR, HFR".to_string()),
                bitrate: Some(1000000),
                audio_bitrate: None,
            },
        ),
        (
            "335",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("1080p HDR, HFR".to_string()),
                bitrate: Some(1500000),
                audio_bitrate: None,
            },
        ),
        (
            "336",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("1440p HDR, HFR".to_string()),
                bitrate: Some(5000000),
                audio_bitrate: None,
            },
        ),
        (
            "337",
            StaticFormat {
                mime_type: r#"video/webm; codecs="VP9""#.to_string(),
                quality_label: Some("2160p HDR, HFR".to_string()),
                bitrate: Some(12000000),
                audio_bitrate: None,
            },
        ),
    ])
});
