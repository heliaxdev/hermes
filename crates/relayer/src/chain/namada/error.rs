use flex_error::{define_error, TraceError};

define_error! {
    Error {
        AddressDecode
            { raw: String }
            |e| { format!("Namada address decoding failed for {}", e.raw) },

        AmountParseError
            [ TraceError<namada_sdk::token::AmountParseError> ]
            |_| { "Amount parse error" },

        DenomNotFound
            { denom: String }
            |e| { format!("No denom for {}", e.denom) },

        Namada
            [ TraceError<namada_sdk::error::Error> ]
            |_| { "Namada error" },

        Query
            [ TraceError<namada_sdk::queries::Error> ]
            |_| { "Query error" },

        BorshDecode
            [ TraceError<std::io::Error> ]
            |_| { "borsh decoding failed" },

        DryRun
            { tx_result: namada_sdk::tx::data::TxResult<String> }
            |e| { format!("Dry run to simulate a transaction failed: {}", e.tx_result) },
    }
}

impl From<Error> for crate::error::Error {
    fn from(error: Error) -> Self {
        Self::namada(error)
    }
}
