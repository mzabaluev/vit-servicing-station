table! {
    api_tokens (token) {
        token -> Binary,
        creation_time -> BigInt,
        expire_time -> BigInt,
    }
}

table! {
    funds (id) {
        id -> Integer,
        fund_name -> Text,
        fund_goal -> Text,
        voting_power_info -> Text,
        voting_power_threshold -> BigInt,
        rewards_info -> Text,
        fund_start_time -> BigInt,
        fund_end_time -> BigInt,
        next_fund_start_time -> BigInt,
    }
}

table! {
    proposals (id) {
        id -> Integer,
        proposal_id -> Text,
        proposal_category -> Text,
        proposal_title -> Text,
        proposal_summary -> Text,
        proposal_problem -> Text,
        proposal_solution -> Text,
        proposal_public_key -> Text,
        proposal_funds -> BigInt,
        proposal_url -> Text,
        proposal_files_url -> Text,
        proposal_impact_score -> BigInt,
        proposer_name -> Text,
        proposer_contact -> Text,
        proposer_url -> Text,
        proposer_relevant_experience -> Text,
        chain_proposal_id -> Binary,
        chain_proposal_index -> BigInt,
        chain_vote_options -> Text,
        chain_voteplan_id -> Text,
    }
}

table! {
    voteplans (id) {
        id -> Integer,
        chain_voteplan_id -> Text,
        chain_vote_start_time -> BigInt,
        chain_vote_end_time -> BigInt,
        chain_committee_end_time -> BigInt,
        chain_voteplan_payload -> Text,
        chain_vote_encryption_key -> Text,
        fund_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(api_tokens, funds, proposals, voteplans,);
