// @generated automatically by Diesel CLI.

diesel::table! {
    AnimTypes (AnimationID) {
        AnimationID -> Integer,
        AnimationType -> Text,
    }
}

diesel::table! {
    CardTypes (TypeID) {
        TypeID -> Integer,
        Type -> Text,
        IsMonster -> Integer,
    }
}

diesel::table! {
    Cards (CardID) {
        CardID -> Integer,
        Name -> Text,
        Description -> Text,
        GuardianStarAID -> Integer,
        GuardianStarBID -> Integer,
        Level -> Integer,
        TypeID -> Integer,
        Attack -> Integer,
        Defense -> Integer,
        Stars -> Integer,
        CardCode -> Integer,
        Attribute -> Integer,
        NameColor -> Integer,
        DescColor -> Integer,
        AbcSort -> Integer,
        MaxSort -> Integer,
        AtkSort -> Integer,
        DefSort -> Integer,
        TypSort -> Integer,
        AISort -> Integer,
        AiGs -> Nullable<Integer>,
    }
}

diesel::table! {
    ChoiceTypes (ChoiceID) {
        ChoiceID -> Integer,
        ChoiceType -> Text,
        TimeCost -> Integer,
    }
}

diesel::table! {
    Decomps (DecompID) {
        DecompID -> Integer,
        AnimationID -> Integer,
        ChoiceID -> Integer,
    }
}

diesel::table! {
    DuelistPools (DuelistPoolID) {
        DuelistPoolID -> Integer,
        DuelistID -> Integer,
        PoolTypeID -> Integer,
        CardID -> Integer,
        Prob -> Integer,
    }
}

diesel::table! {
    Duelists (DuelistID) {
        DuelistID -> Integer,
        Name -> Text,
        IsMage -> Bool,
        HandSize -> Integer,
        LowLPThreshold -> Integer,
        CriticalDeckSize -> Integer,
        MaxFusionLength -> Integer,
        MaxImproveLength -> Integer,
        SpellProbability -> Text,
        AttackProbability -> Text,
        FIND_BEST_COMBO_TD -> Integer,
        IMPROVE_MONSTER_TD -> Integer,
        SET_MAGIC_TD -> Integer,
        FIND_BEST_COMBO_NO_TD -> Integer,
        IMPROVE_MONSTER_NO_TD -> Integer,
        SET_MAGIC_NO_TD -> Integer,
    }
}

diesel::table! {
    Equips (EquipID) {
        EquipID -> Integer,
        EquipCardID -> Integer,
        CardID -> Integer,
    }
}

diesel::table! {
    FieldEffects (FieldEffectID) {
        FieldEffectID -> Integer,
        FieldTypeID -> Integer,
        CardTypeID -> Integer,
        Relation -> Integer,
    }
}

diesel::table! {
    FieldTypes (FieldTypeID) {
        FieldTypeID -> Integer,
        Name -> Text,
    }
}

diesel::table! {
    FixedAdvs (FixedAdvID) {
        FixedAdvID -> Integer,
        AnimationID -> Integer,
        InitialSeedIndex -> Integer,
    }
}

diesel::table! {
    Fusions (FusionID) {
        FusionID -> Integer,
        Card1ID -> Integer,
        Card2ID -> Integer,
        ResultCardID -> Integer,
    }
}

diesel::table! {
    GuardianStars (GuardianStarID) {
        GuardianStarID -> Integer,
        Name -> Text,
    }
}

diesel::table! {
    PoolTypes (PoolTypeID) {
        PoolTypeID -> Integer,
        PoolType -> Text,
    }
}

diesel::table! {
    Rituals (RitualID) {
        RitualID -> Integer,
        RitualCardID -> Integer,
        Card1ID -> Integer,
        Card2ID -> Integer,
        Card3ID -> Integer,
        ResultCardID -> Integer,
    }
}

diesel::table! {
    StarterGroups (PoolID) {
        PoolID -> Integer,
        SampleSize -> Integer,
    }
}

diesel::table! {
    StarterPools (StarterPoolID) {
        StarterPoolID -> Integer,
        PoolID -> Integer,
        CardID -> Integer,
        Prob -> Integer,
    }
}

diesel::table! {
    VariableAdvs (VariableAdvID) {
        VariableAdvID -> Integer,
        AnimationID -> Integer,
        InitialSeedIndex -> Integer,
        AdvanceSize -> Integer,
    }
}

diesel::joinable!(Cards -> CardTypes (TypeID));
diesel::joinable!(Decomps -> AnimTypes (AnimationID));
diesel::joinable!(Decomps -> ChoiceTypes (ChoiceID));
diesel::joinable!(DuelistPools -> Cards (CardID));
diesel::joinable!(DuelistPools -> Duelists (DuelistID));
diesel::joinable!(DuelistPools -> PoolTypes (PoolTypeID));
diesel::joinable!(FieldEffects -> FieldTypes (FieldTypeID));
diesel::joinable!(FixedAdvs -> AnimTypes (AnimationID));
diesel::joinable!(StarterPools -> Cards (CardID));
diesel::joinable!(StarterPools -> StarterGroups (PoolID));
diesel::joinable!(VariableAdvs -> AnimTypes (AnimationID));

diesel::allow_tables_to_appear_in_same_query!(
    AnimTypes,
    CardTypes,
    Cards,
    ChoiceTypes,
    Decomps,
    DuelistPools,
    Duelists,
    Equips,
    FieldEffects,
    FieldTypes,
    FixedAdvs,
    Fusions,
    GuardianStars,
    PoolTypes,
    Rituals,
    StarterGroups,
    StarterPools,
    VariableAdvs,
);
