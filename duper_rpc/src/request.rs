use duper::{
    DuperValue,
    serde::{error::DuperSerdeError, ser::to_duper},
};
use serde_core::Serialize;

use crate::{Error, RequestId};

#[derive(Debug, Clone)]
pub enum Request {
    Single(RequestCall),
    Batch(Vec<RequestCall>),
}

#[derive(Debug, Clone)]
pub enum RequestCall {
    Valid {
        id: Option<RequestId>,
        method: String,
        params: DuperValue<'static>,
    },
    Invalid {
        id: Option<RequestId>,
        error: Error,
    },
}

#[derive(Default)]
pub struct RequestBuilder;

pub struct RequestBuilderSingle {
    call: Result<RequestCall, DuperSerdeError>,
}

pub struct RequestBuilderBatch {
    calls: Result<Vec<RequestCall>, DuperSerdeError>,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn request0(self, method: String, id: Option<RequestId>) -> RequestBuilderSingle {
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![],
                },
            }),
        }
    }

    pub fn request1<T1>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![t1.static_clone()],
                },
            }),
        }
    }

    pub fn request2<T1, T2>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![t1.static_clone(), t2.static_clone()],
                },
            }),
        }
    }

    pub fn request3<T1, T2, T3>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t3 = match to_duper(t3) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![t1.static_clone(), t2.static_clone(), t3.static_clone()],
                },
            }),
        }
    }

    pub fn request4<T1, T2, T3, T4>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t3 = match to_duper(t3) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t4 = match to_duper(t4) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![
                        t1.static_clone(),
                        t2.static_clone(),
                        t3.static_clone(),
                        t4.static_clone(),
                    ],
                },
            }),
        }
    }

    pub fn request5<T1, T2, T3, T4, T5>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t3 = match to_duper(t3) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t4 = match to_duper(t4) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t5 = match to_duper(t5) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![
                        t1.static_clone(),
                        t2.static_clone(),
                        t3.static_clone(),
                        t4.static_clone(),
                        t5.static_clone(),
                    ],
                },
            }),
        }
    }

    pub fn request6<T1, T2, T3, T4, T5, T6>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t3 = match to_duper(t3) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t4 = match to_duper(t4) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t5 = match to_duper(t5) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t6 = match to_duper(t6) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![
                        t1.static_clone(),
                        t2.static_clone(),
                        t3.static_clone(),
                        t4.static_clone(),
                        t5.static_clone(),
                        t6.static_clone(),
                    ],
                },
            }),
        }
    }

    pub fn request7<T1, T2, T3, T4, T5, T6, T7>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
        t7: &T7,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
        T7: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t3 = match to_duper(t3) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t4 = match to_duper(t4) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t5 = match to_duper(t5) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t6 = match to_duper(t6) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t7 = match to_duper(t7) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![
                        t1.static_clone(),
                        t2.static_clone(),
                        t3.static_clone(),
                        t4.static_clone(),
                        t5.static_clone(),
                        t6.static_clone(),
                        t7.static_clone(),
                    ],
                },
            }),
        }
    }

    pub fn request8<T1, T2, T3, T4, T5, T6, T7, T8>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
        t7: &T7,
        t8: &T8,
    ) -> RequestBuilderSingle
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
        T7: Serialize,
        T8: Serialize,
    {
        let t1 = match to_duper(t1) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t2 = match to_duper(t2) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t3 = match to_duper(t3) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t4 = match to_duper(t4) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t5 = match to_duper(t5) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t6 = match to_duper(t6) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t7 = match to_duper(t7) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        let t8 = match to_duper(t8) {
            Ok(value) => value,
            Err(error) => return RequestBuilderSingle { call: Err(error) },
        };
        RequestBuilderSingle {
            call: Ok(RequestCall::Valid {
                id,
                method,
                params: DuperValue::Tuple {
                    identifier: None,
                    inner: vec![
                        t1.static_clone(),
                        t2.static_clone(),
                        t3.static_clone(),
                        t4.static_clone(),
                        t5.static_clone(),
                        t6.static_clone(),
                        t7.static_clone(),
                        t8.static_clone(),
                    ],
                },
            }),
        }
    }
}

impl RequestBuilderSingle {
    pub fn build(self) -> Result<Request, DuperSerdeError> {
        self.call.map(Request::Single)
    }

    pub fn request0(self, method: String, id: Option<RequestId>) -> RequestBuilderBatch {
        match self.call {
            Ok(call) => RequestBuilderBatch {
                calls: Ok(vec![
                    call,
                    RequestCall::Valid {
                        id,
                        method,
                        params: DuperValue::Tuple {
                            identifier: None,
                            inner: vec![],
                        },
                    },
                ]),
            },
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request1<T1>(self, method: String, id: Option<RequestId>, t1: &T1) -> RequestBuilderBatch
    where
        T1: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![t1.static_clone()],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request2<T1, T2>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![t1.static_clone(), t2.static_clone()],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request3<T1, T2, T3>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![
                                    t1.static_clone(),
                                    t2.static_clone(),
                                    t3.static_clone(),
                                ],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request4<T1, T2, T3, T4>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![
                                    t1.static_clone(),
                                    t2.static_clone(),
                                    t3.static_clone(),
                                    t4.static_clone(),
                                ],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request5<T1, T2, T3, T4, T5>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![
                                    t1.static_clone(),
                                    t2.static_clone(),
                                    t3.static_clone(),
                                    t4.static_clone(),
                                    t5.static_clone(),
                                ],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request6<T1, T2, T3, T4, T5, T6>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t6 = match to_duper(t6) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![
                                    t1.static_clone(),
                                    t2.static_clone(),
                                    t3.static_clone(),
                                    t4.static_clone(),
                                    t5.static_clone(),
                                    t6.static_clone(),
                                ],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request7<T1, T2, T3, T4, T5, T6, T7>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
        t7: &T7,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
        T7: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t6 = match to_duper(t6) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t7 = match to_duper(t7) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![
                                    t1.static_clone(),
                                    t2.static_clone(),
                                    t3.static_clone(),
                                    t4.static_clone(),
                                    t5.static_clone(),
                                    t6.static_clone(),
                                    t7.static_clone(),
                                ],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request8<T1, T2, T3, T4, T5, T6, T7, T8>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
        t7: &T7,
        t8: &T8,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
        T7: Serialize,
        T8: Serialize,
    {
        match self.call {
            Ok(call) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t6 = match to_duper(t6) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t7 = match to_duper(t7) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t8 = match to_duper(t8) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                RequestBuilderBatch {
                    calls: Ok(vec![
                        call,
                        RequestCall::Valid {
                            id,
                            method,
                            params: DuperValue::Tuple {
                                identifier: None,
                                inner: vec![
                                    t1.static_clone(),
                                    t2.static_clone(),
                                    t3.static_clone(),
                                    t4.static_clone(),
                                    t5.static_clone(),
                                    t6.static_clone(),
                                    t7.static_clone(),
                                    t8.static_clone(),
                                ],
                            },
                        },
                    ]),
                }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }
}
impl RequestBuilderBatch {
    pub fn build(self) -> Result<Request, DuperSerdeError> {
        self.calls.map(Request::Batch)
    }

    pub fn request0(self, method: String, id: Option<RequestId>) -> RequestBuilderBatch {
        match self.calls {
            Ok(mut calls) => {
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request1<T1>(self, method: String, id: Option<RequestId>, t1: &T1) -> RequestBuilderBatch
    where
        T1: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![t1.static_clone()],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request2<T1, T2>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![t1.static_clone(), t2.static_clone()],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request3<T1, T2, T3>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![t1.static_clone(), t2.static_clone(), t3.static_clone()],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request4<T1, T2, T3, T4>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            t1.static_clone(),
                            t2.static_clone(),
                            t3.static_clone(),
                            t4.static_clone(),
                        ],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request5<T1, T2, T3, T4, T5>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            t1.static_clone(),
                            t2.static_clone(),
                            t3.static_clone(),
                            t4.static_clone(),
                            t5.static_clone(),
                        ],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request6<T1, T2, T3, T4, T5, T6>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t6 = match to_duper(t6) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            t1.static_clone(),
                            t2.static_clone(),
                            t3.static_clone(),
                            t4.static_clone(),
                            t5.static_clone(),
                            t6.static_clone(),
                        ],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request7<T1, T2, T3, T4, T5, T6, T7>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
        t7: &T7,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
        T7: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t6 = match to_duper(t6) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t7 = match to_duper(t7) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            t1.static_clone(),
                            t2.static_clone(),
                            t3.static_clone(),
                            t4.static_clone(),
                            t5.static_clone(),
                            t6.static_clone(),
                            t7.static_clone(),
                        ],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }

    pub fn request8<T1, T2, T3, T4, T5, T6, T7, T8>(
        self,
        method: String,
        id: Option<RequestId>,
        t1: &T1,
        t2: &T2,
        t3: &T3,
        t4: &T4,
        t5: &T5,
        t6: &T6,
        t7: &T7,
        t8: &T8,
    ) -> RequestBuilderBatch
    where
        T1: Serialize,
        T2: Serialize,
        T3: Serialize,
        T4: Serialize,
        T5: Serialize,
        T6: Serialize,
        T7: Serialize,
        T8: Serialize,
    {
        match self.calls {
            Ok(mut calls) => {
                let t1 = match to_duper(t1) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t2 = match to_duper(t2) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t3 = match to_duper(t3) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t4 = match to_duper(t4) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t5 = match to_duper(t5) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t6 = match to_duper(t6) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t7 = match to_duper(t7) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                let t8 = match to_duper(t8) {
                    Ok(value) => value,
                    Err(error) => return RequestBuilderBatch { calls: Err(error) },
                };
                calls.push(RequestCall::Valid {
                    id,
                    method,
                    params: DuperValue::Tuple {
                        identifier: None,
                        inner: vec![
                            t1.static_clone(),
                            t2.static_clone(),
                            t3.static_clone(),
                            t4.static_clone(),
                            t5.static_clone(),
                            t6.static_clone(),
                            t7.static_clone(),
                            t8.static_clone(),
                        ],
                    },
                });
                RequestBuilderBatch { calls: Ok(calls) }
            }
            Err(error) => RequestBuilderBatch { calls: Err(error) },
        }
    }
}
