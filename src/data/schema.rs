// @generated automatically by Diesel CLI.

diesel::table! {
    anim_types (animation_id) {
        animation_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    card_types (type_id) {
        type_id -> Integer,
        name -> Text,
        is_monster -> Integer,
    }
}

diesel::table! {
    cards (card_id) {
        card_id -> Integer,
        name -> Text,
        description -> Text,
        guardian_star_a_id -> Integer,
        guardian_star_b_id -> Integer,
        level -> Integer,
        type_id -> Integer,
        attack -> Integer,
        defense -> Integer,
        stars -> Integer,
        card_code -> Integer,
        attribute -> Integer,
        name_color -> Integer,
        desc_color -> Integer,
        abc_sort -> Integer,
        max_sort -> Integer,
        atk_sort -> Integer,
        def_sort -> Integer,
        typ_sort -> Integer,
        ai_sort -> Integer,
        ai_gs -> Nullable<Integer>,
    }
}

diesel::table! {
    choice_types (choice_id) {
        choice_id -> Integer,
        name -> Text,
        time_cost -> Integer,
    }
}

diesel::table! {
    decomps (decomp_id) {
        decomp_id -> Integer,
        animation_id -> Integer,
        choice_id -> Integer,
    }
}

diesel::table! {
    duelist_pools (duelist_pool_id) {
        duelist_pool_id -> Integer,
        duelist_id -> Integer,
        pool_type_id -> Integer,
        card_id -> Integer,
        prob -> Integer,
    }
}

diesel::table! {
    duelists (duelist_id) {
        duelist_id -> Integer,
        name -> Text,
        is_mage -> Bool,
        hand_size -> Integer,
        low_lp_threshold -> Integer,
        critical_deck_size -> Integer,
        max_fusion_length -> Integer,
        max_improve_length -> Integer,
        spell_probability -> Text,
        attack_probability -> Text,
        find_best_combo_td -> Integer,
        improve_monster_td -> Integer,
        set_magic_td -> Integer,
        find_best_combo_no_td -> Integer,
        improve_monster_no_td -> Integer,
        set_magic_no_td -> Integer,
    }
}

diesel::table! {
    equips (equip_id) {
        equip_id -> Integer,
        equip_card_id -> Integer,
        card_id -> Integer,
    }
}

diesel::table! {
    field_effects (field_effect_id) {
        field_effect_id -> Integer,
        field_type_id -> Integer,
        card_type_id -> Integer,
        relation -> Integer,
    }
}

diesel::table! {
    field_types (field_type_id) {
        field_type_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    fixed_advs (fixed_adv_id) {
        fixed_adv_id -> Integer,
        animation_id -> Integer,
        initial_seed_index -> Integer,
    }
}

diesel::table! {
    fusions (fusion_id) {
        fusion_id -> Integer,
        card1_id -> Integer,
        card2_id -> Integer,
        result_card_id -> Integer,
    }
}

diesel::table! {
    guardian_stars (guardian_star_id) {
        guardian_star_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    pool_types (pool_type_id) {
        pool_type_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    rituals (ritual_id) {
        ritual_id -> Integer,
        ritual_card_id -> Integer,
        card1_id -> Integer,
        card2_id -> Integer,
        card3_id -> Integer,
        result_card_id -> Integer,
    }
}

diesel::table! {
    starter_groups (pool_id) {
        pool_id -> Integer,
        sample_size -> Integer,
    }
}

diesel::table! {
    starter_pools (starter_pool_id) {
        starter_pool_id -> Integer,
        pool_id -> Integer,
        card_id -> Integer,
        prob -> Integer,
    }
}

diesel::table! {
    variable_advs (variable_adv_id) {
        variable_adv_id -> Integer,
        animation_id -> Integer,
        initial_seed_index -> Integer,
        advance_size -> Integer,
    }
}

diesel::joinable!(cards -> card_types (type_id));
diesel::joinable!(decomps -> anim_types (animation_id));
diesel::joinable!(decomps -> choice_types (choice_id));
diesel::joinable!(duelist_pools -> cards (card_id));
diesel::joinable!(duelist_pools -> duelists (duelist_id));
diesel::joinable!(duelist_pools -> pool_types (pool_type_id));
diesel::joinable!(field_effects -> field_types (field_type_id));
diesel::joinable!(fixed_advs -> anim_types (animation_id));
diesel::joinable!(starter_pools -> cards (card_id));
diesel::joinable!(starter_pools -> starter_groups (pool_id));
diesel::joinable!(variable_advs -> anim_types (animation_id));

diesel::allow_tables_to_appear_in_same_query!(
    anim_types,
    card_types,
    cards,
    choice_types,
    decomps,
    duelist_pools,
    duelists,
    equips,
    field_effects,
    field_types,
    fixed_advs,
    fusions,
    guardian_stars,
    pool_types,
    rituals,
    starter_groups,
    starter_pools,
    variable_advs,
);
