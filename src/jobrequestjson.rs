use serde_json::{json, Value};

// Build a request: https://users.rust-lang.org/t/make-nested-json-objects-in-serde-reqwest/54479/2
// Structs are obviously more type safe, but this is faster
pub fn job_request_json(limit: u32) -> Value {
    json!({
        "structuredQuery": {
            "from": [
                {
                    "collectionId": "featured_jobs"
                }
            ],
            "where": {
                "compositeFilter": {
                    "op": "AND",
                    "filters": [
                        {
                            "fieldFilter": {
                                "field": {
                                    "fieldPath": "date"
                                },
                                "op": "LESS_THAN",
                                "value": {
                                    "integerValue": "1668808196225"
                                }
                            }
                        },
                        {
                            "fieldFilter": {
                                "field": {
                                    "fieldPath": "websites"
                                },
                                "op": "ARRAY_CONTAINS",
                                "value": {
                                    "stringValue": "Rust"
                                }
                            }
                        }
                    ]
                }
            },
            "orderBy": [
                {
                    "field": {
                        "fieldPath": "date"
                    },
                    "direction": "DESCENDING"
                },
                {
                    "field": {
                        "fieldPath": "__name__"
                    },
                    "direction": "DESCENDING"
                }
            ],
            "limit": limit,
            "startAt": {
                "before": false,
                "values": [
                    {
                        "integerValue": "1664324451910"
                    },
                    {
                        "referenceValue": "projects/scalajobs-56919/databases/(default)/documents/featured_jobs/WcRpJ34txO6c9w82t9Bi"
                    }
                ]
            }
        }
    })
}

