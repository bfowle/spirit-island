# Steam Spirit Island engine: state model, randomness, rule resolution, information sets, and decision points

Scope: the Handelabra Steam build of Spirit Island, read statically. Targets:
`SpiritIslandEngine.dll` (assembly 1.0.9776.27799, SHA-256 `eac8e2ed…fe750`, MVID `a05049d9-2a7f-45bb-8dfa-f011f043456a`, 5,432 TypeDefs, 21,420 fields, 36,017 methods), `EngineCommon.dll` (SHA-256 `4693cc4a…b22`), the four `EndGame--*.txt` action logs, and the `.dat` saves. The game reports itself as "Spirit Island version 3.4.1 build 840 prod" with Unity 2022.3.62f3 ([Player.log](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/Player.log)).

Labels: **observed** means the fact is visible in metadata, IL, an embedded resource, a log line, or a save. **inferred** means it is reasoned from names or structure and not confirmed by IL or a log. This document states no tabletop rule from memory. Where the code does not show something, it says so.

Link key, used as sources below:
- DLL = [SpiritIslandEngine.dll](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- EC = [EngineCommon.dll](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/EngineCommon.dll)
- LOG-A…D = [EndGame--8584242708433629839.txt](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt), [EndGame--8584246831828257582.txt](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt), [EndGame--8584246844275351304.txt](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt), [EndGame--8584248061651619129.txt](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584248061651619129.txt)
- SAVE = [save-Feedback-004-0131-SlowPower.dat](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/save-Feedback-004-0131-SlowPower.dat)
- DEF/<name> = the embedded JSON resource `Handelabra.SpiritIsland.Engine.Definitions.<name>.json` inside the DLL (11 manifest resources; extracted read-only)

## Q1. Which engine types model the game state, and what fields do they hold?

### Takeaway
All state hangs from one serializable root, `Handelabra.SpiritIsland.Engine.Model.Game` (83 fields). It holds the island graph (`Island` → `IslandBoard` → `Land`), a flat list of pieces, spirits with presence tracks and element counters, and every card zone as a `Location`. It also holds a list of status effects (the modifier system), a typed journal of every rule action, and the RNG state. Decks are ordered `List<Card>` fields on `Location`. No separate deck object exists. The save files serialize exactly these backing fields.

### Cited Findings

#### Evidence method (observed)
- Types and fields were read from the DLL metadata with REA `inspect_managed_members`. For type-filtered selections, REA reported `coverage.state = complete`. The unfiltered call hit REA's 64 MB representation budget. A read-only Python metadata parse (`dnfile` 0.18 + `dncil`, in a scratch venv; no assembly loaded or executed) was used for the full type list, constant values, field-RVA array initializers, and static CIL disassembly of specific methods. Every IL claim below names the method that was disassembled. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Namespaces: `…Engine` (166 types: Land, Island, IslandBoard, Piece, enums, `Constants`), `…Engine.Model` (270 types: Game, Spirit, cards, status effects, journal entries), `…Engine.Model.Definitions` (35 types), and `…Engine.Controller` (1,815 types plus 43 per-spirit/per-content sub-namespaces: Adversaries 158, EventCards 414, FearCards 225, PowerCards 723, Scenarios 198, GrowthOptions 122, …). The model and controller layers are separate: each model type has a `…Controller` that holds the rule logic. — [DLL metadata](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### `Model.Game : BaseModel` (observed, all 83 fields; types as declared)
- RNG and identity: `RNG : Handelabra.SerializableRandom`, `InitialRNG : SerializableRandom`, `RandomSeed : int?`, `SavedModelVersion : int`, `CurrentModelVersion : int [static]`, `DifficultyRating : int?`, `ViewData : string`. — [DLL `Model.Game` fields](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Clock and phase: `TurnIndex : int`, `StateIndex : int`, `State : Model.State` (fields `PhaseType`, `StateType`, `Namespace`), `_states : List<State>`, `OverrideNextState : State`, `WaveNumber : int`, `SecondWaveAfterGameSetupStage : SecondWaveSetupStage?`, `NextActivationNumber : int`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Board: `Island : Island`, `BoardLayout`, `InitialBoardIdentifiers : List<string>`, `InitialBoardLayout`, `InitialNumberOfBoards : int`, `ResetBoardLayoutNextWave : bool`, `_pieces : List<Piece>` (every piece in the game, wherever it is). — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Spirits: `_spirits : List<Spirit>`, `_previousWaveSpirits : List<Spirit>`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Card zones, each a `Model.Location`: `MajorPowerDeck`, `MajorPowerDiscard`, `MinorPowerDeck`, `MinorPowerDiscard`, `FearDeck`, `FearEarned`, `FearDiscard`, `_invaderCardSlots : List<Location>`, `ExploreSlot`, `BuildSlot`, `RavageSlot`, `DiscardSlot`, `OutOfGame`, `RevealLocation`, `EventDeck`, `EventDiscard`, `PlayArea`, `DummyLocation`. Single cards: `BlightCard : BlightCard`, `TerrorLevelCard : TerrorLevelCard`, plus `_previousBlightCards`, `_replacementBlightCards`, and `_allCards : List<Card>`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Fear: `FearGenerated : int`, `FearRequiredForFearCard : int`. Invaders: `HighestInvaderStageReached : int`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Modifiers: `Adversary`, `_adversaries : List<Adversary>`, `Scenario`, `_gameModifiers : List<GameModifier>`, `_statusEffects : List<StatusEffect>`, `_practiceModeTypes : List<PracticeModeType>`, `IsUsingTokens`, `IsUsingEvents`, `IsGameMultiplayer`, `AskAdjacenciesForRuleActions`, and allowed-pool lists: `AllowedMinorPowerCardIdentifiers`, `AllowedMajorPowerCardIdentifiers`, `AllowedEventCardIdentifiers`, `AllowedFearCardIdentifiers`, `AllowedBlightCardIdentifiers`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- History and outcome: `Journal : Model.Journal` (`_journalEntries : List<JournalEntry>`), `EndingResult : EndingResult?`, `PreviousGameData`. Instance counters exist for each piece kind: Explorer, Town, City, Dahan, Blight, Beast, Wild, Disease, Strife, ScenarioToken, SpecialToken, Badland, Vitality, Thief. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Board and location types (observed)
- `Model.BaseModel`: `Identifier : string`, `_propertiesInt/_propertiesBool/_propertiesString : List<ModelProperty>` (a generic key/value bag; controllers store flags such as `"BadlandInvaderDamageKey"` here), `_baseDefinition`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.Location : BaseModel`: `Owner : BaseModel`, `LocationType : LocationType`, `_pieces : List<Piece>`, `_cards : List<Card>` (the ordered deck or stack), `ShuffleCount : int`, `_title : string`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `IslandLocation : Location`: `_defense : List<Defend>`, `PlanningIndicator : LandIndicator?`, `WerePiecesChangedThisRuleAction : Stack<bool>`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Land : IslandLocation`: `OwnerBoard : IslandBoard`, `DamageTaken : int`, `_defense : List<Defend>`, `HomeOfSpirit : Spirit`, `PlanningIndicator`, `_combinedLands : List<Land>`, `WerePiecesChangedThisRuleAction`, `SuppressedBy : Land`, `HealthModifiers : List<HealthModifier>`. REA confirms 9 fields on Land and 3 on IslandLocation, so Land re-declares three of them. — [DLL; REA selection Land+IslandLocation = 12 fields](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Island : BaseModel`: `_islandBoards`, `_outOfGameIslandBoards`, `LandAdjacencyMatrix : bool[,]`, `LandMatrixDimensionSize : int`, `_boardLayout`, `LandCorners : List<LandCorner>`, `CornerInstanceIdentifierCount`, `LandDistanceCache : int[,]`, `IsArchipelagoAllowed`, `RequireLandCorners`, `_islets : List<Islet>`. `IslandBoard`: `OwnerIsland`, `_lands : List<Land>`, `BoardNumber : int`. `LandCorner`: `OwnerIsland`, `LandsForCorner : List<Land>`, `SideMarkByLand : List<Tuple<Land,int>>`, `InstanceIdentifier`, `LandIdentifiers`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Enums: `LandType` {Jungle=0, Mountain, Ocean, Sands, Wetland, Coastal, SaltDeposits=6}. `LocationType` has 27 values, among them Land=4, Blight=5, Explore=6, Build=7, Ravage=8, PresenceTrackNode=9, FearEarned=19, EventDeck=22, ExploreRevealed=25. — [DLL constants](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Pieces and tokens (observed)
- `Piece : BaseModel`: `_title`, `OwnerLocation : Location`, `OwnerScenario`, `IsMarked : bool`, `_associatedCards`. `PieceType` {Presence=0, Blight=1, IslandPiece=2, Token=3, Incarna=4}. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.IslandPiece : Piece` (invaders and Dahan): `MaxHealth : int`, `DamageTaken : int`, `IslandPieceType`, `InstanceIdentifier : int`, `TokenLocation : Location` (tokens attached to the piece, such as strife), `AssociatedNumber : int?`, `ShowAssociatedNumber`, `HealthModifiers : List<HealthModifier>`. `IslandPieceType` {Dahan=0, Explorer=1, Town=2, City=3, Thief=4}. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.Token : Piece`: `TokenType`, `InstanceIdentifier`, `IsFaceDown : bool`, `NumberValue : int?`, `SpecialModel`. `TokenType` {Beast=0, Wilds=1, Disease=2, Strife=3, Scenario=4, Badlands=5, Special=6, Vitality=7}. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.Presence : Piece`: `OwnerSpirit`, `InstanceIdentifier`. `Model.BlightPiece : Piece`: `InstanceIdentifier`. `Model.Incarna : Piece`: `OwnerSpirit`, `IsEmpowered`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.HealthModifier`: `OwnerIslandPiece`, `OwnerLand`, `Source`, `Amount : int`. `Model.Defend`: `OwnerLand`, `Source`, `Amount : int`, `DefendSingleLand : bool`, `OwnerIslandLocation`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Spirits, presence tracks, elements, powers (observed)
- `Model.Spirit` (45 fields): `Energy : int`, `RemainingCardPlays : int`, `_presenceTrack`, `_presenceTrackEnergy`, `_presenceTrackCardPlays : List<PresenceTrackNode>`, `_presence : List<Presence>`, `Incarna`, `_innatePowers : List<Power>`, `_specialRules`, zones `PlayArea`, `Deck`, `Discard`, `Hand`, `PowerProgressionDeck`, `OutOfGame`, `RevealLocation`, `SpecialRuleLocation` (all `Location`). Element counters are `ElementsSun…ElementsAnimal` and `ElementMarkersSun…ElementMarkersAnimal` (8 + 8 ints), plus `_elements`, `_elementMarkers : ElementSet`. Also `SpiritPhaseStatus : SpiritPhaseStatus?`, `AlwaysSpiritGrowthGroup`, `_spiritGrowthGroups`, `GainEnergyGrowth`, `Aspect`, `StartingIslandBoard`, `SpiritUsesPowerProgression`, and `PowersBeingResolved`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.PresenceTrackNode : Location`: `OwnerSpirit`, `_nextNodes : List<PresenceTrackNode>` (tracks form a graph, not just an array), `IsStartingNode`, `GrowthOption`. `TrackNodeType` has 22 values: EnergyPerTurn, CardPlaysPerTurn, Element, ReclaimCards, CardPlaysPerTurnAndReclaimOne, …, PermanentElement=18, ImpendingEnergyPerTurn=21. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `ElementType` {Sun=0, Moon, Fire, Air, Water, Earth, Plant, Animal=7}. `Model.ElementSet`: `InterchangeableElementTypes`, `_dict : Dictionary<ElementType,int>`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Model.Power`: `Owner`, `_powerElementOptions : List<PowerElementOption>` (each holds `_costElements : ElementSet`, the innate or threshold requirement), `_repeatList`, `Speed : bool`, `_spiritRuleList`, `_modifiedByList`, `IsImmediateRepeating`. `PowerType` {Major=0, Minor=1, Unique=2, Innate=3}. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `GrowthOption` (13 fields: `UsedThisTurn`, `FinishedThisTurn`, `OverrideGrowthType`, `OwnerGrowthGroup`, `GrowthOptionChoices`, `NumberOfTimesGrowthUsedThisTurn`, …). `GrowthGroup`: `GrowthOptions`, `HasBeenUsedThisTurn`. `GrowthType` has 59 values. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Cards (observed)
- `Model.Card : BaseModel`: `OwnerLocation`, `IsFlipped : bool`, `IsSpecificallyPlaced : bool`, `InstanceIdentifier : int`, `IsKnown : bool`. Subclasses: `PowerCard` (`OwnerSpirit`, `OriginalOwnerSpirit`, `Power`), `InvaderCard` (`OverrideStage : int?`), `FearCard` (`ResolvedAtTerrorLevel : int?`), `EventCard` (`_eventSections`), `BlightCard` (`BlightLocation : Location`, which holds the blight pool), `TerrorLevelCard` (no fields), `ReminderCard`, and `GameModifier` (base of `Adversary`, `AdversaryLevel`, `Scenario`). — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Adversary`: `Game`, `Level : int?`, `_adversaryLevels`, `AdversaryLocation`, `_townMaxHealthAdjustment : int?`, `_cityMaxHealthAdjustment : int?`. `Scenario`: `_scenarioLands`, `_tokens`, `ScenarioLocation`, `_benefits`, `ScenarioPowers`, `ScenarioGrowthOptions`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Status effects: the modifier system (observed)
- `Model.StatusEffect`: `Source`, `NumberOfUses : int?`, `StatusEffectType`, `HandlesOwnExpiration`, `StartTurnIndex : int?`, `DecisionMakers`, `InstanceIdentifier`, `RuleActionIndex`, `PersistAcrossGames`, `SuppressDisplayText`. 46 subclasses, one per `StatusEffectType` value (RepeatPower=0 … ModifyPieceDamage=45). Examples: `DefendStatusEffect` (`DefendAmount`, `DefendPerPiece`, `PieceTypes`, `WithinRange`, `IsPermanent`, `AffectsAllLands`, …), `AdjustHealthStatusEffect` (`HealthModifier`, `PieceType`, `PerStrife`, `MinimumHealth`, …), `AdjustRavageDamageStatusEffect` (20 conditions such as `IfLandHasPresence`, `PerInvaderOfType`, `IfLandHasNoStrife`, `OnlyDamageToLand`, `OnlyDamageToDahan`, `AddInvaderStageAsDamage`), `ModifyDahanCounterattackStatusEffect` (`OverrideDahanCounterattackTiming`, `OverrideDamageAmount`), `PreventBlightCascadeStatusEffect`, `SkipInvaderActionsStatusEffect`, `CancelActionStatusEffect`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Rules are modified through an "ask" protocol. `GameController` and `BaseController` define about 116 `AskFor…` / `AskIf…` methods. These are keyed by the `AskType` enum (138 values), for example `ShouldBlightCascade`=78, `ChangeFearGeneratedForDestroy`=70, `ModifyFearRequiredForFearCard`, `NumberOfPiecesToBuild`, `IslandPieceTypeToBuildInLand`, `ModifyRavageDamage`, `ExploreRange`, `InvaderStage`, `LandMatchesInvaderCard`. Each rule computes a default and then calls `AskControllersForFirstAnswer` or `AskControllersForAllAnswers`. — [DLL IL `GameController::ShouldBlightCascade`, `::GetFearGeneratedForDestroyedPiece`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Journal: a typed event history (observed)
- `Model.JournalEntry`: `Source`, `TurnIndex`, `StateIndex`, `State`, `IsUndoRelated`, `_relatedEntries`, `JournalIndex`, `RuleActionJournalIndex`. There are about 60 subclasses. The useful ones for statistics:
  - `RavageLandJournalEntry` (`LandRavaged`, `RavageSlotInvaderCard`, `WasCancelled`)
  - `RavageCompleteJournalEntry` (`DidAddBlight`, `LandsBlighted`)
  - `BuildLandJournalEntry` (`LandBuilt`, `BuildSlotInvaderCard`, `WasCancelled`)
  - `ExploreJournalEntry` (`LandExplored`, `ExploreSlotInvaderCard`, `WasSuccessfullyExplorer`)
  - `AddBlightJournalEntry` (22 fields, among them `IsFromCascade`, `IsFromRavage`, `RavageDamageToLand`, `WillCascade`, `ShouldFlipBlightCard`, `AmountFromBlightCard`, `AmountFromBox`, `LandMaxHealth`)
  - `DealDamageJournalEntry` (`DamageSources`, `DamageTargets`, `DamageDistribution : Dictionary<string,int>`, `IsDamagePretend`)
  - `AdjustFearGeneratedJournalEntry` (`AmountOfFearGenerated`, `TargetLand`, `DestroyedPieces`)
  - `GainPowerCardJournalEntry` (`PowerCardGained`, `PowerCardsRevealedBeforeShuffle`, `PowerCardsNotGained`, `WasLocationShuffled`)
  - `UseActivationJournalEntry` (`ActivationType : string`, `SelectionIndex`, `ChoiceIdentifiers : string[]`, `Pieces`, `Location`, `Spirit`, `Power`)

  — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- The 45 `EndGame--*.dat` files contain these journal classes. A January 2025 file declares `RavageLandJournalEntry`, `RavageCompleteJournalEntry`, `ExploreJournalEntry`, `GainPowerCardJournalEntry`, `ResolveFearCardJournalEntry`, `UseActivationJournalEntry`, and others. — [EndGame--8584654268616841938.dat](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584654268616841938.dat)

#### Constants and turn structure (observed)
- `Constants` (562 fields, 339 with literal values):
  - Health and damage: `DahanHealth=2`, `ExplorerHealth=1`, `TownHealth=2`, `CityHealth=3`, `ThiefHealth=1`, `LandHealth=2`.
  - Fear: `FearGeneratedFromDestroyingTowns=1`, `FearGeneratedFromDestroyingCities=2`, `FearGeneratedFromDestroyingThieves=1`, `NumberOfFearTokensPerSpirit=4`, `NumberOfFearCards=9`.
  - Decks: `NumberOfStage1InvaderCards=3`, `NumberOfStage2InvaderCards=4`, `NumberOfStage3InvaderCards=5`, `NumberOfCardsToRevealWhenGainingPowerCards=4`, `NumberOfMajorPowerCards=22`, `NumberOfMinorPowerCards=36`.
  - Other: `NumberOfPresencePerSpirit=13`, `MaxRange=100`, `CombinedAdversaryDifficultyFactor=0.625`.

  — [DLL Constant table](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Constants.NumberOfLevelFearCardsByTerrorLevel = {3,3,3}` (read from the field-RVA initializer). `Constants.StateOrdering` = [PlayerSetup, InvadersStartingAction, SpiritPhase, FastPower, BlightedIsland, Event, Fear, HighImmigration, Ravage, Build, Explore, AdvanceInvaderCards, SlowPower, TimePasses]. This is the per-turn state sequence after setup. — [DLL `Constants::.cctor` + FieldRVA](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `StateType` has 17 values (PlayerSetup=0 … Event=16). `PhaseType` {StartOfGame, Spirit, FastPower, Invader, SlowPower, TimePasses}. `SpiritPhaseStatus` {Grow, GainEnergy, PlayPayPower, Complete} are the sub-steps of SpiritPhase. The log shows `ChangeSpiritPhaseStatusAction … ToSpiritPhaseStatus=Grow`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-A](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt)
- `EndingResult` {WinInvadersDestroyed=0, WinTerrorVictory, WinSacrificeVictory, WinScenario, LoseSpiritDestroyed, LoseTimeRunsOut, LoseTooMuchBlight, LoseAdversary, LoseScenario, LoseAbandoned, LoseSpiritUnique=10}. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Save files (observed)
- The MS-NRBF saves store these backing fields: `<RNG>k__BackingField`, `<InitialRNG>…`, `<RandomSeed>…`, `SeedArray`, `inext`, `inextp`, `<IsKnown>…`, `<IsFlipped>…`, `<FearGenerated>…`, `<FearRequiredForFearCard>…`, `<OverrideStage>…`, `<ResolvedAtTerrorLevel>…`, and others. — [SAVE](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/save-Feedback-004-0131-SlowPower.dat)
- The schema drifts between versions. The April 2026 save names assembly `1.0.9602.20319`; its `Land` derives directly from `Location`, and it has `SuppressLandData` where the current DLL has `SuppressedBy`. A January 2025 EndGame file names `1.0.9116.39151`. The current DLL is `1.0.9776.27799`. — [SAVE](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/save-Feedback-004-0131-SlowPower.dat); [EndGame--8584654268616841938.dat](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584654268616841938.dat)

### Inferences
- **Model state vector (inferred).** The minimal decision-relevant state per time step (TurnIndex, StateType, SpiritPhaseStatus) has these parts:
  - Per land: piece multiset by `IslandPieceType`, damage taken, Defend total, health modifiers, tokens by `TokenType`, blight count, presence by spirit.
  - Per spirit: Energy, RemainingCardPlays, hand, discard and play-area cards, presence-track coverage, the element counts, and element markers.
  - Global: `FearGenerated`, `FearRequiredForFearCard`, fear-deck size above each terror divider, `FearEarned` count, current terror level, blight-card pool count and side, invader slot contents, invader-deck stage backs, active status effects, and adversary or scenario level.

  The RNG fields and deck orders are excluded because they are hidden (see Q4).
- The journal entries make the EndGame `.dat` files a per-action panel dataset. Examples: Ravage outcome per land per turn (`RavageCompleteJournalEntry.DidAddBlight`), blight source (card or box, cascade or ravage), and fear per source. That data could support empirical calibration across the 45 archived games (inferred; not decoded here).

### Gaps
- Nested and compiler-generated types (lambdas, iterator state machines) were not listed field by field. Only the top-level model types are inventoried.
- No save was decoded field by field (MS-NRBF parsing was out of scope). Save-content claims rest on `strings` output.
- The drift between the three engine versions was not mapped field by field.

## Q2. Where does randomness enter, and how is the RNG used?

### Takeaway
All in-game randomness comes from one stream, `Game.RNG`, a `Handelabra.SerializableRandom`. It is a serializable copy of the .NET Framework subtractive generator. It is seeded once from the clock and its full internal state is written into every save. Invader and fear decks are built at setup by sequential uniform sampling without replacement; there is no later shuffle, so the order is fixed at setup. Event, minor, and major decks get a Fisher–Yates shuffle at setup. During play the RNG is consumed only by power-deck reshuffles and a short list of adversary, scenario, and power effects. The static scan found no other RNG.

### Cited Findings
- **Seeding (observed).** `Game::SetupRNG`: if no seed is supplied, `RandomSeed = DateTime.Now.ToFileTimeUtc()` (stored as `int?`). Then `RNG = new SerializableRandom(seed)` and `InitialRNG = new SerializableRandom(seed)`. Only `Game::.ctor` reads `InitialRNG`. — [DLL IL `Model.Game::SetupRNG`; xref scan](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Generator (observed).**
  - `SerializableRandom` keeps `SeedArray`, `inext`, `inextp`, and an `MSEED` string. `InternalSample` indexes with constant 56 and wraps at 2147483647. `Sample()` multiplies by 4.656612875245797e-10 (that is, 1/int.MaxValue). `Next(int)`, `Next(int,int)` (with `GetSampleForLargeRange`), `NextDouble`, and `NextBytes` are present.
  - The parameterless constructor seeds from `Environment.TickCount`, but the engine always passes a seed.

  — [EC IL `Handelabra.SerializableRandom::*`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/EngineCommon.dll)
- **Shuffle algorithm (observed).** `GenericListExtensions.Shuffle(list, rng)` loops `i = Count−1 … 1`, with `j = rng.Next(i+1)` and `Swap(i, j)`. This is the Durstenfeld form of Fisher–Yates. — [EC IL `Handelabra.GenericListExtensions::Shuffle`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/EngineCommon.dll)
- **Draw-without-replacement (observed).** `GenericEnumerationExtensions.TakeRandom(source, n, rng)` repeats n times: `k = rng.Next(0, list.Count)`, take element k, remove it. `TakeRandomFirstOrDefault` = `TakeRandom(1).FirstOrDefault()`. — [EC IL `TakeRandom`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/EngineCommon.dll)
- **Location shuffle (observed).** `Location::ShuffleCards(rng)` does `ShuffleCount++`, then `_cards.Shuffle(rng)`, then `SetIsKnown(false)` on every card. — [DLL IL `Model.Location::ShuffleCards`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Complete list of RNG consumers (observed; static IL xref over all 36,017 methods).**
  - `Game.get_RNG` is read by:
    - Setup: `Game::.ctor`, `CreateBlightCard`, `CreateEventDeck`, `CreateFearDeck`, `CreateInvaderDeck`, `CreateMajorPowerDeck`, `CreateMinorPowerDeck`, `GameModifier::MakeSetupFearDeckAdjustments`
    - Play: `ShuffleLocationAction::DoAction`, `AddNewCardAction::CreateNewCard`, `GameController::GetRandomBoardIdentifierFromAllowedBoards`
    - Adversaries: France `PerformEscalationAfterExploreResponse`, Sweden `Escalation`
    - Powers: Fractured Days `VisionsOfAShiftingFuture…ReturnAndShuffleCardsResponse`
    - Scenarios: A Diversity of Spirits (`DetermineNextSpiritToUsePower`, `GetRandomOrderOfSpirits`), Despicable Theft `AddThiefFromPool`, Powers Long Forgotten `PerformAdditionalInvaderSetup`
  - `GameController::ShuffleLocation` is called from `ReshufflePowerDeck`, `DiscardTopCard`, and Ward the Shores `ShuffleFearDiscardResponse`.
  - The scan found no use of `System.Random`, `UnityEngine.Random`, or `Guid.NewGuid` in the engine. `DateTime.Now` appears only in `SetupRNG`.

  — [DLL static xref](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Setup order (observed).** `Game::.ctor` calls, in order: `SetupRNG` → `CreateAdversaries` → `CreateScenario` → `CreateIsland` → `CreateSpirits` → `SetUpInitialPieces` → `CreateEventDeck` → `CreateBlightCard` → `CreateFearValues` → `CreateLocations` → `CreateInvaderDeck` → `CreateFearDeck` → `CreatePowerCards` (→ `CreateMajorPowerDeck`, `CreateMinorPowerDeck`) → `CreateStates`. — [DLL IL `Model.Game::.ctor`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Invader deck (observed).**
  - `Game::CreateInvaderDeck` first gathers `GetInvaderCardsRemovedBeforeRandomizing()` from the adversaries (in reverse order). It then calls `PieceFactory.CreateRandomInvaderCards(RNG, stage, n)` three times: (1, 3), (2, 4), (3, 5).
  - Each call filters `Constants.AvailableInvaderCardIdentifiers` to the stage, removes the excluded cards, and picks cards one at a time with `TakeRandom(1)`.
  - The cards go into `ExploreSlot` as stacked blocks I, II, III. Then each adversary runs `MakeSetupInvaderDeckAdjustments` (it reorders cards by position and can set `IsKnown`). With Branch & Claw content, `CommandBeasts` reminder cards are inserted at positions found from the stage-II cards.

  — [DLL IL `Model.Game::CreateInvaderDeck`, `PieceFactory::CreateRandomInvaderCards`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Invader pool (observed).** Stage I has 4 cards: Jungle1, Mountain1, Sands1, Wetland1. Stage II has 5: Coastal2, plus Jungle2, Mountain2, Sands2, Wetland2, each flagged `"escalation": true`. Stage III has 6 two-terrain cards: JungleSands3, JungleWetland3, MountainJungle3, MountainWetland3, SandsMountain3, SandsWetland3. So one card per stage is never drawn. — [DEF/InvaderCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **The "Invader Deck" is `Game.ExploreSlot` (observed).** `Game::CreateInvaderSlots` builds `ExploreSlot` with LocationType 6 and title `'Invader Deck'`. It builds `BuildSlot` as `'Build Space'`, `RavageSlot` as `'Ravage Space'`, and `DiscardSlot` as `'Invader Discard'`. The log confirms that a revealed explore card is the flipped top card of that stack: `FlipCardAction … CardBeingFlipped=Mountain1` is followed by `AdvanceInvaderCardsAction: CardMovements=(Stage I Mountain, Invader Deck, Build Space)`. — [DLL IL `Model.Game::CreateInvaderSlots`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-A](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt)
- **Adversary invader-deck changes (observed in the definitions).** `removeInvaderCard` (stage number), `moveInvaderCard` [from, to], `moveMultipleInvaderCards` [3, 2] (Russia L4: "111-2-3-2-3-2-3-2-33"), and `replaceInsteadOfMove` (Scotland L4) are positional operations. Examples: Brandenburg-Prussia L2 moves one stage-III card between I and II; L3/L5 remove an extra stage-I card; L4 removes an extra stage-II card; L6 removes all stage-I cards. Habsburg Mining L4 swaps in Salt Deposits. Habsburg Livestock L5 inserts a reminder card under the top 5 invader cards. `MakeSetupInvaderDeckAdjustments` is not an RNG consumer in the xref. — [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Fear deck (observed).**
  - `Game::CreateFearDeck` sets the per-terror-level counts to `{3,3,3}` plus the chosen adversary level's `fearCardModifiers` string. It reads only `AdversaryLevels.ElementAtOrDefault(Level−1)`, so each level's string is cumulative (e.g. England L3 `"1/2/1"`). Scenario `FearCardModifiers` are added too.
  - It draws the total N with `PieceFactory.CreateRandomFearCards(RNG, N, include, allowed)`, which calls `TakeRandom(1)` repeatedly.
  - It then stacks the deck in draw order: `Take(n1)`, then the Terror Level II divider, `Take(n2)`, then the Terror Level III divider, then the rest, then the final `TerrorVictory` card. There is no separate shuffle; `MakeSetupFearDeckAdjustments` may consume RNG.
  - The definitions hold 50 fear cards: Base 15, Branch & Claw 15, Nature Incarnate 9, Jagged Earth 6, Promo Pack 2 5.

  — [DLL IL `Model.Game::CreateFearDeck`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/FearCardList, DEF/TerrorLevelCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Event deck (observed).** `Game::CreateEventDeck` takes `AvailableEventCardIdentifiers` minus `Scenario.RemoveEventCardsBeforeRandomizing`, runs `CreateEventCards`, adds them to `EventDeck`, then calls `EventDeck.ShuffleCards(RNG)`. The definitions hold 62 events: Jagged Earth 30, Branch & Claw 23, Nature Incarnate 9. — [DLL IL `Model.Game::CreateEventDeck`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/EventCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Power decks (observed).**
  - `Game::CreateMajorPowerDeck` and `::CreateMinorPowerDeck` create the allowed list (default `Constants.AvailableMajor/MinorPowerCardIdentifiers`), add it to the deck, and call `ShuffleCards(RNG)`.
  - The definitions hold 321 power cards. Minor: Base 36, Branch & Claw 32, Jagged Earth 33. Major: Base 22, Branch & Claw 21, Jagged Earth 23, Nature Incarnate 12. Unique: 142.
  - During play, `GameController::RevealCardsShufflingIfNeeded` → `ReshufflePowerDeck` → `ShuffleLocation` reshuffles the discard when the deck runs short. Its message reads "`… has no cards left to reshuffle into the …`".

  — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/PowerCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Blight card (observed).**
  - `PieceFactory::CreateBlightCard` picks with `TakeRandom(1)` from the allowed blight cards. `Game::CreateBlightCard` drops `Constants.BlightCardsWithTwoBlightPerBoardOrImmediateEffects` under a condition tied to events or tokens being off. The exact condition was not traced.
  - The blight pool on the card is `HealthyIslandNumberPerStartingBoard × InitialNumberOfBoards + 1` (IL: `mul; ldc.i4.1; add`).
  - In the definitions, 24 real blight cards all have `healthyIslandNumberPerStartingBoard = 2`, with flipped values 1–5. The placeholder `"Blight"` (no blight card) has 5.

  — [DLL IL `Model.Game::CreateBlightCard`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/BlightCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **RNG state is in every save (observed).** The save holds `Handelabra.SerializableRandom` with `SeedArray`, `inext`, `inextp`, and `<RandomSeed>`. — [SAVE](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/save-Feedback-004-0131-SlowPower.dat)

### Inferences
- **Distributions (inferred, assuming the generator is close to uniform).**
  - Within each invader stage, the deck order is a uniformly random ordered k-subset of the n-card pool: (n, k) = (4, 3), (5, 4), (6, 5). The stages are concatenated and then rearranged by position. For a player, each unseen card of a known stage is exchangeable with every other card of that stage, including the removed one.
  - The fear deck is a uniformly random ordered N-subset of the allowed fear pool, cut at fixed divider positions.
  - Event, minor, and major decks are uniform random permutations of their allowed sets.
- The process is effectively frozen at setup. Every invader, fear, and event outcome is fixed by `Game::.ctor`. Afterwards the RNG moves only on power-deck reshuffles and the effects listed above. So "randomness" during play is epistemic (hidden order) rather than fresh sampling. Exceptions are reshuffles, France and Sweden escalations, and the listed scenarios.
- Anyone who reads a save can, in principle, predict every future reshuffle and escalation pick: they need the serialized `SeedArray/inext/inextp` and must replay the consumption order. Undo (`save-Undo-*.dat`) restores the RNG state with the rest of the model, so undoing does not re-roll a reshuffle (inferred; the RNG travels with the snapshot).
- The ordered sampling in `TakeRandom` is equivalent to a uniform random permutation truncated to k. This holds exactly for Next(0, m) only up to the generator's slight modulo bias, which `Sample()`-based scaling keeps tiny.

### Gaps
- `MakeSetupFearDeckAdjustments` and the adversary deck-adjustment bodies were not fully disassembled. Which adversary uses RNG in fear-deck setup is unconfirmed.
- No reshuffle of the event deck was found. `EventStateController` logs "The Event Deck is empty so an Event cannot be resolved!" instead. Whether a scenario refills it was not checked.
- Not checked: whether `RandomSeed` truncation from a 64-bit FILETIME to `int?` loses entropy, and how the cast is done.

## Q3. How does the engine resolve Ravage, Build, Explore, damage, Defend, health, Dahan counterattack, blight and cascade, fear and terror, presence and elements, power gain, events, tokens, and adversary/scenario modifiers?

### Takeaway
The rules run as coroutines on `…Controller.*Action` classes. Each phase is a `*StateController.RunState`. Numbers come from piece properties and `Constants`, then pass through the AskType modifier protocol. Most outcomes given the board are deterministic. Players supply only the choices listed in Q5: cascade destination, Dahan counterattack distribution, presence to destroy, ordering, and so on.

### Cited Findings

#### Turn and phase flow (observed)
- State order is given by `Constants.StateOrdering` (Q1). `TimePassesAction` does the following, in order:
  1. Discards played power cards and flips them.
  2. Calls `SetDamageTaken(0)` on all damage targets (`DamageTargetsClearingDamage`), which is the heal.
  3. Calls `IslandLocation::ClearDefense` on every land and clears the badland damage keys.
  4. Expires status effects.
  5. Calls `AdjustCardPlays` to `GetCardPlaysPerTurn`.
  6. Calls `LoseElements`, except presence-track elements and `PermanentElementsStatusEffect` elements.
  7. Resets growth data.
  8. Calls `Game::IncreaseTurnIndex`.

  — [DLL IL `TimePassesAction+<DoActionCoroutine>d__25::MoveNext`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `AdvanceInvaderCardsStateController.RunState` advances the invader card slots toward `DiscardSlot`. `AdvanceAllInvaderSlots` marks the `ExploreSlot` top card to advance. Log: `AdvanceInvaderCardsAction: CardMovements=(Stage I Mountain, Invader Deck, Build Space)`. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-A](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt)
- `GameController::GetCurrentInvaderStage` returns the stage of the top `ExploreSlot` card (`GetInvaderStageFromCard`), or 3 if the deck is empty, and then applies `AskForInvaderStage`. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Health and damage values (observed)
- `PieceFactory::CreateIslandPiece` switches on `IslandPieceType` to set max health: Dahan 2, Explorer 1, Town `2 + Σ GameModifier.TownMaxHealthAdjustment`, City `3 + Σ CityMaxHealthAdjustment`, Thief 1. England L5 sets `townMaxHealthAdjustment: 1`, `cityMaxHealthAdjustment: 1`. — [DLL IL `PieceFactory::CreateIslandPiece`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `IslandPiece::GetCurrentMaxHealth` = `MaxHealth + Σ HealthModifiers.Amount`, floored at 1 (`ldc.i4.0; bgt … ldc.i4.1`). `GetCurrentHealth = GetCurrentMaxHealth − DamageTaken`. `IslandPieceController::GetCurrentHealth` also subtracts `PretendDamage`, which is used for previews. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `Land::get_MaxHealth` returns the literal 2. `Land::GetCurrentMaxHealth = max(1, 2 + Σ HealthModifiers)`. `Land::get_CurrentHealth = GetCurrentMaxHealth − DamageTaken`. — [DLL IL `Land::get_MaxHealth`, `::GetCurrentMaxHealth`, `::get_CurrentHealth`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Damage a piece deals: `IslandPiece::get_DamageToDeal` returns `NormalMaxHealth` (so Explorer 1, Town 2, City 3, Dahan 2), and null for a Thief. `GameController::GetDamageForIslandPiece` applies `AskForOverridenDamageForPiece` and `AskForModifiedDamageForPiece` (additive), with a floor of 0. Sweden L3 sets `townDamageAdjustment: 1`, `cityDamageAdjustment: 2`; its text reads "Town deal 3 Damage. City deal 5 Damage". — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Log confirmation: `DamageAmountsBySource=[City2-3]`, `[Explorer1-1]`, and Dahan counterattack `[Dahan5-2, Dahan6-2]`. — [LOG-B](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt); [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)

#### Ravage (observed unless marked)
- `GameController::GetInvaderDamageForLand` sums `GetDamageForIslandPiece` over `GetPiecesParticipatingInRavage(…, includeInvaders:true, includeDahan:false)`. It adds `GetRavageDamageModification` (when requested) and subtracts `IslandLocationController.DefenseValue` when the defense flag is set. — [DLL IL `GameController::GetInvaderDamageForLand`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `RavageAction` keeps separate totals: `AmountOfDamageFromInvadersToLand`, `…ToDahan`, `…ToOtherTargets`, plus `LandDefense`, `DamageAdjustmentsBySource`, `NumberOfDahanDestroyed`, `DidAddBlight`, and `LandsWithBlightAdded`. User strings: "`--> Invaders adjust their damage output by {0} (Dahan: {1} / Land: {2}).`" and "`Invaders are dealing {0} to targets other than the Land or Dahan.`" — [DLL `RavageAction` fields and #US strings](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- The land and the Dahan each receive the invaders' damage total. One explorer's 1 damage logged `DamageDistribution=Dahan5 => 1, JungleG7 => 1`. — [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)
- The ravage coroutine calls, in order:
  1. `GetPossibleDahanCounterattackTiming` → (if several timings) `ActivateSelectEffect`.
  2. `GetIslandLocationDefenseValue` → `set_LandDefense`.
  3. `GetRavageTargetsForLand`.
  4. `makeDahanDamageDistribution` → `DistributeDamage`.
  5. `UpdatePiecesParticipatingInRavage` and token removal (`GetApplicableTokensAtLocation`, `RemoveTokens`).
  6. Per-source damage adjustments.
  7. `HandleRavageInvaderStepModifiers`.
  8. `DealDamage`.
  9. `set_NumberOfDahanDestroyed`, `set_DidAddBlight`.
  10. `RecordRavageCompleteJournalEntry`.

  — [REA call edges `<DoActionCoroutine>d__57.MoveNext`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Dahan counterattack (observed).** `GetPossibleDahanCounterattackTiming` defaults to `AfterInvaders` (enum 2) and lets controllers add or replace timings. The enum is `DahanCounterattackTiming` {BeforeInvaders, Simultaneous, AfterInvaders}. Strings: "`{Dahan} deal Damage before Invaders`", "`… simultaneously`", "`… after Invaders`". In the log, counterattack damage is a `DealDamageActivation` with `DamageSources=[Dahan5/Dahan6] … DamageAmount=4`, which is 2 per Dahan. — [DLL IL + #US](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)
- **Land damage → blight (observed).** In `DealDamageAction.AfterAction`, for each land damage target, if `landTarget.CurrentHealth > 0` is false, the engine calls `GameController.AddBlight(…)` with a literal 1 in the argument list. It then sets `DidAddBlight` and calls `Land.SetDamageTaken(0)`. So one ravage adds one blight when land damage ≥ current land health (2 + modifiers, floor 1), and land damage resets. — [DLL IL `DealDamageAction+<AfterAction>d__139::MoveNext`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Damage allocation (observed names; mechanism inferred).** `GameController::DistributeDamage` has 37 parameters (`maxDamagePerTarget`, `damageShouldBeEfficient`, `maxDamageByLocation`, …). `GetTargetDamagePriority(target, damageRemaining, …, invertDahanDamagePriority, …)` ranks targets. Damage to pieces is applied through `DealDamageAction` and then `DestroyPiecesAction` when health reaches 0 (log: `DestroyPiecesAction … PiecesBeingDestroyed=Explorer1`). — [DLL signatures](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-B](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)

#### Defend (observed)
- Defend is a list of `Model.Defend {Amount, Source, DefendSingleLand}` on each `IslandLocation._defense`, plus `DefendStatusEffect`. `GetIslandLocationDefenseValue` collects defends over the location's combined lands, concatenates `AskForAdjustedDefends`, and filters them. `ReduceDefenseStatusEffect` and `AskForDefenseReduction` exist. `TimePasses` clears defense. — [DLL IL `GameController::GetIslandLocationDefenseValue`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Build (observed unless marked)
- `GameController::GetIslandPieceTypeToBuildInLand` counts towns (lambda `IsTown`) and cities (lambda `IsCity`) among `GetPiecesToConsiderForBuild`, each plus an optional modifier. If towns > cities it returns City (3), otherwise Town (2). The result then passes through `AskForIslandPieceTypeToBuildInLand`. — [DLL IL `GetIslandPieceTypeToBuildInLand`, `<>c::<…>b__747_0/1`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `GetNumberOfPiecesToBuildInLand` defaults to 1, overridable by `AskForNumberOfPiecesToBuild`. Habsburg L2 builds 2 Towns instead of a City inland, per its definition text. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Which lands build: `ResolveInvaderCardAction` holds `LandsMatchingInvaderCard`, `CanInvaderActionBePerformedInLand`, `BuildInLand`, and `RavageInLand`. The IL of the "land must have invaders" test was not read (gap). — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Explore (observed)
- `GameController::GetLandsToExploreFrom` defines the base source predicate (lambda `b__0`): the land is not isolated AND (`IsOcean` OR `HasInvaders(explorers:null, towns:true, cities:true)`). It then adds `AskForExploreSourcesToAdd` (AskType 30) and removes `AskForExploreSourcesToRemove` (31). If the card's land types include Coastal (5), the ordering is special-cased. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `IsSourceLandInExploreRange` returns true if source = target. Otherwise it tests whether the target is within `GetExploreRange()` of a source. `GetExploreRange = 1 + AskForExploreRange`. `ExploreAction.NumberOfPiecesToAddByType : Dictionary<IslandPieceType,int>` (default 1 explorer, inferred). Scotland L1 replaces the explorer with a town in coastal lands, per its text. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Blight, cascade, blight card (observed)
- `AddBlightAction` takes blight from the Blight Card's `BlightLocation`, unless `IsFromBox`. If the card holds fewer blight than needed while healthy-side up, it sets `ShouldFlipBlightCard`. `WillCascade = ShouldBlightCascade(…)`. After adding, it runs `CheckForNoMoreBlight` and `IsGameOver`. — [DLL IL `AddBlightAction+<DoActionCoroutine>d__102`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `ShouldBlightCascade` defaults to `islandLocation.HasBlight`, read before the blight is added, or to "override amount > 0". Any controller answering `ShouldBlightCascade` (AskType 78) can veto it. `PreventBlightCascadeStatusEffect` exists. — [DLL IL `GameController::ShouldBlightCascade`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `AddBlightAction.AfterAction` runs `HandleBlightDestroyingPresence` (`ShouldAddingBlightDestroyPresence` → `SelectPiecesAndDoAction("{Presence}")` → `DestroyPieces`). Then, if `WillCascade && AllowCascade`, it runs `GameController.CascadeBlight`. `CascadeBlight` takes `GetAdjacentIslandLocations`, requires the blight card to hold more than 0 blight, then calls `CreateSelectLocationActivation` + `ActivateExclusive` to let the player choose, and finally `AddBlight(selected, isFromCascade:true)`. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Log: `AddBlightAction … WillCascade: True, IsFromCascade: False` → `SelectLocationActivation: Reason=CascadingBlight, … Choices: MountainA1, WetlandA2, JungleA3, WetlandA5` → `AddBlightAction … Destination: Jungle A3, WillCascade: False, IsFromCascade: True`. — [LOG-B](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)
- `BlightedIslandStateController.RunState` calls `ResolveBlightCardEffect` when the card is flipped and `ImplementsBlightedIslandEffect`. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Fear and terror (observed)
- **Fear threshold.** `Game::CreateFearValues`: `FearGenerated = 0`. `perSpirit = 4`, raised to 5 when `#boards == #spirits + 1`. `FearRequiredForFearCard = perSpirit × NumberOfSpirits`, then each adversary's `MakeSetupFearPoolAdjustments` (England L6: `extraFearPerPlayer: 1`). At run time, `GetFearRequiredForFearCard` adds every controller's ModifyFearRequiredForFearCard answer (AskType 75). — [DLL IL `Model.Game::CreateFearValues`, `GameController::GetFearRequiredForFearCard`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Generation loop.** `GameController.GenerateFear` repeatedly adds `min(remaining, FearRequired − FearGenerated)` via `AdjustFearGeneratedAction`. Whenever `FearGenerated ≥ FearRequired`, it calls `EarnFearCards(1, reset)`, which moves the top card of `FearDeck` to `FearEarned` and resets `FearGenerated` to 0. It carries overflow, with a 100-iteration safety ("GenerateFear got stuck in a loop"). — [DLL IL `GameController+<GenerateFear>d__441`, `<EarnFearCards>d__442`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Fear from destruction.** `IslandPieceController::get_FearFromDestroy`: Town 1, City 2, Thief 1, everything else 0 (Explorers, Dahan). `GetFearGeneratedForDestroyedPiece` then applies ChangeFearGeneratedForDestroy (AskType 70). — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Resolution.** `FearStateController` → `ResolveAllFearCards`: `FlipStack(FearEarned)`, then resolves the top card in order (`ResolveFearCard`) until empty or game over. `FearCard.ResolvedAtTerrorLevel` is recorded. A card above the current terror level logs "`… could not resolve fear card {1} because the game's terror level is at {2}`". `GetFearRequiredForNextTerrorLevel = (cards above the next terror divider) × FearRequiredForFearCard − FearGenerated`. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Terror levels.** TL I victory conditions are [NoExplorers, NoTowns, NoCities]; TL II [NoTowns, NoCities]; TL III [NoCities]; TL 4 is "Terror Victory". `HaveSpiritsMetVictoryConditions` checks `IslandController.AllInvaders` for each listed type. Log: `SetTerrorLevelCardAction: Source: Visions of Fiery Doom`. — [DEF/TerrorLevelCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DLL IL `GameController::HaveSpiritsMetVictoryConditions`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-B](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)

#### Presence, energy, card plays, elements, innates (names observed; formulas not read)
- Methods: `GameController.GetSpiritEnergyPerTurn(spirit, allowModifiers)`, `GetSpiritCardPlaysPerTurn(spirit, out isFixed, asOfTurnIndex)`, `GetEnergyPerTurnFromTrackNode`, `GetCardPlaysPerTurnFromTrackNode`, `GetSpiritUncoveredEnergyPerTurn`, `GetSpiritUncoveredCardPlaysPerTurn`, `HandlePresenceTrackNodeCoroutines`, `DoesPowerMeetElementalThreshold(…, elementsInUse, includePotentialElements, …)`, `GetElementsForSpirit`, `GainElements`, and `LoseElements`. AskTypes `SpiritEnergyPerTurnAdjustment`, `SpiritCardPlaysPerTurnAdjustment`, and `DoesPowerMeetElementalThreshold` modify them. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Spirit and power data (tracks, innate thresholds, growth) live in `DEF/SpiritList` (371 KB) and `DEF/PowerCardList` (fields `elementTypes`, `elementOptions`, `energyCost`, `speed`, `rangeDistance`, `targetType`). — [DLL resources](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Power-card gain (observed)
- `NumberOfCardsToRevealWhenGainingPowerCards = 4`. `RevealPowerCardsAndSelectCardsToGain` → `RevealCardsShufflingIfNeeded` → `ReshufflePowerDeck` when needed. `GainPowerCardAction` records `PowerCardsNotGained` and `WasDeckShuffled`. Log: `SelectCardActivation: CardChoices=DriftDownIntoSlumber, VisionsOfFieryDoom, GiftOfConstancy, PurifyingFlame, SelectedCard=Visions of Fiery Doom, SelectCardReason=GainPower`. This is preceded by `SelectLocationActivation: Reason=PowerDeckToDrawFrom` (14 times). — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-B](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)

#### Events (observed)
- `EventStateController.RunState`: if `ShouldResolveEventState`, it calls `ResolveEventCard` on `EventDeck.TopCard`, then `MoveCard` → `EventDiscard`. The Event state comes after BlightedIsland and before Fear in `StateOrdering`. Event cards have main, token, and Dahan sections (`GetMainEventSection`, `GetTokenEventSection`, `GetDahanEventSection`; `EventSectionType` has 16 values). Some sections are choices made by players (`ChoiceEvent`, `EventChoice`). — [DLL IL `EventStateController+<RunState>d__2`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Tokens (names observed; numeric effects not read)
- Ravage removes tokens through `GetApplicableTokensAtLocation` → `RemoveTokens` (call edges in the ravage coroutine). `InvaderAction` has `DoesTokenCancelAction`, `_tokenToCancel`, and `TokenSelfCancelResponse`, so tokens can cancel invader actions. Strife: `InvadersWithStrifeDealDamageToOtherInvaders`, `EachInvaderTakesDamagePerStrife`, `AdjustHealthStatusEffect.PerStrife`, `ChangeRavageParticipation`. Badlands: `DealDamageAction.UsedInvaderBadlands/UsedDahanBadlands`, `HandleUnresolvedBadlands`, and the `"BadlandInvaderDamageKey"` and `"BadlandDahanDamageKey"` properties cleared at Time Passes. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Adversary and scenario modifiers (observed)
- Adversary data per level includes `fearCardModifiers`, invader-deck edits, setup piece adds (`addTownToLandNumberOnEachBoard`, `addCityToLandNumberOnEachBoard`, …), health and damage adjustments, `difficulty`, `stageIIEscalationBody`, and `additionalLossCondition`. There are 8 adversaries: Brandenburg-Prussia, England, France, Habsburg Monarchy, Habsburg Mining, Russia, Sweden, Scotland. — [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- The run-time hooks are `AdversaryController.Escalation`, `GetNumberOfEscalationsThatShouldOccur`, `PerformAdditionalInvaderSetup`, and many AskFor overrides. Escalation is flagged on the stage-II terrain cards (`"escalation": true`). Log: `Source: Stage II Jungle {Escalation}`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)

### Inferences
- **Ravage transition, conditional on the board and modifiers (inferred from the observed parts).** Let D = Σ damage over participating invaders, plus adjustments. The land takes max(0, D − Defend) (the clamp at 0 is not seen in this method). If that is ≥ land health h (default 2), one blight is added. The Dahan take D, distributed by the engine's priority rule. The surviving Dahan then deal 2 each to the invaders (timing AfterInvaders by default).

  Blight cascades when the land already had blight; the player picks the adjacent land, so it is a decision, not a random draw. A cascade is itself an AddBlight and can chain while each landing land already had blight and the card still holds blight.

  This makes "land blights on ravage" a deterministic function of the counts (explorers, towns, cities, Dahan, Defend, and modifiers). Odds over future turns therefore come only from which lands the hidden invader cards will hit and from the players' own actions.
- Total fear needed for Terror Victory ≈ (total fear cards) × FearRequiredForFearCard − FearGenerated. In a base game without adversary that is 9 × 4·S with S spirits, or 9 × 5·S when there is one extra board (inferred from the formulas above).
- Blight-loss risk: the healthy-side pool B₀ = 2·boards + 1 for every real blight card. When it empties the card flips, and the flipped pool depends on `flippedNumberPerStartingBoard` (1–5). The flipped-pool multiplier was not read from IL.

### Gaps
- Not read in IL:
  - the exact Build and Ravage eligibility test (the "land has invaders" condition) and the ravage participation default for strife'd invaders
  - the flipped-side blight count formula (`GetNumberOfBlightForFlippedBlightCard`)
  - the loss condition for `LoseTimeRunsOut` (what happens when the invader deck runs out)
  - token effect amounts (Wilds, Disease, Beasts, Badlands)
  - whether Defend also reduces damage to Dahan
  - the zero clamp on land damage
  - the energy and card-play formulas from presence tracks
- How `GetTargetDamagePriority` orders Dahan and invader targets (efficient versus inefficient allocation) was not decoded. The signature shows `invertDahanDamagePriority` and `damageShouldBeEfficient` flags exist.

## Q4. What is hidden and what is public to the player at each moment (information sets)?

### Takeaway
Hidden information sits in the order of `Location._cards` for FearDeck, FearEarned (face down until resolved), EventDeck, ExploreSlot (the invader deck), and the Minor/Major decks, and in the RNG state. The engine tracks player knowledge with `Card.IsKnown` and `Card.IsFlipped`. Revealing an unknown card clears the undo files, so a reveal is an irreversible commit point. Everything on the board, in hands, discards, and play areas is open. The text logs leak hidden card names.

### Cited Findings
- `Card` carries `IsFlipped` (face-up) and `IsKnown`. `Location::ShuffleCards` sets `IsKnown = false` on every card after shuffling. `CardController::SetIsKnown` is called from `MoveCardAction::DoAction`, `MoveCardsAction::DoAction`, and France `PerformAdditionalInvaderSetup`. — [DLL IL + xref](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- `CardController::get_ShouldClearSavesAfterFlip` returns `!IsKnown`. `FlipCardAction::ShouldClearSavesAfterAction` uses it. `GameController::SuggestClearSaves` is called from `RevealCardsShufflingIfNeeded` (power reveals), blight-card flipped effects, Fractured Days' reveal, and others. Log pattern: `FlipCardAction … CardBeingFlipped=Mountain1` followed by `***** CLEARING UNDO FILES *****` (117 clear events across the 4 logs). — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-A](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt)
- Earned fear cards are named in the log when earned, before resolution: `MoveCardAction: CardBeingMoved=WaryOfTheInterior, To=Fear Earned, From=Fear Deck`. They are flipped later at the Fear state (`FlipCardAction: Source=ResolveFearCardAction-, CardBeingFlipped=WaryOfTheInterior`). — [LOG-B](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)
- The invader deck is the `ExploreSlot` card list (title "Invader Deck"). The explore card is revealed by `FlipCardAction` during the Explore resolution; the setup explore shows `Source=ResolveInvaderCardAction-, CardBeingFlipped=Mountain1`. Build and Ravage slot cards were revealed in earlier turns and are public. — [DLL IL `Game::CreateInvaderSlots`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-A](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt)
- The RNG (`SeedArray`, `inext`, `inextp`) and `RandomSeed` are serialized into each save. — [SAVE](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/save-Feedback-004-0131-SlowPower.dat)
- The engine keeps a "pretend" damage layer (`IsDamagePretend`, `TakePretendDamage`, `LandPreview`, `GetRavagePreviewForLand`, `GetInvaderActionPreviewForLand`, `IsTerrorWinPossible(ravagePreviews)`) to show the player previews of deterministic outcomes. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

### Inferences
- Information set by phase (inferred from the turn order and the reveal points above):
  - **Spirit phase and Fast powers.** Known: the board; the Ravage and Build slot cards (face-up); the stage of the next invader card (its back); the count left in each invader stage; the event discard; the fear discard; the count of FearEarned cards (not their identities, under the tabletop convention); FearGenerated; the terror level; the blight card side and pool count. Unknown: the next explore card's terrain, the top event, the next fear cards, and the next power cards.
  - **Event step.** The top event is revealed and resolved. Event effects may depend on the board state at that moment.
  - **Fear step.** The FearEarned cards are flipped and resolved at the current terror level.
  - **Ravage, then Build.** Fully determined given the board, except for player choices (cascade, presence destruction, ordering).
  - **Explore.** The ExploreSlot top card flips. This is the main per-turn random reveal for the invader forecast.
  - **Gain Power.** 4 cards are revealed from the chosen deck, then one is chosen.
- For forecasting, the public predictive distribution of the next explore card is uniform over the unseen cards of the next card's stage: pool minus cards already seen, as in the companion spec. That stage is visible from the back, `InvaderCard` stage. This holds because within-stage order came from uniform sampling (Q2). Adversary positional edits change which stage sits where but not the within-stage exchangeability (inferred).
- Because the undo files are cleared on any unknown reveal, a player cannot "peek and undo" in the game. A companion that reads `save-*.dat` would break this. It must therefore drop `RNG/InitialRNG/RandomSeed` and deck orders (consistent with the companion spec §5).

### Gaps
- It was not verified whether the game UI shows FearEarned identities to the player before resolution (the log does). This affects whether the companion should treat earned fear identities as hidden. The `IsKnown` handling in `MoveCardAction` was not decoded.
- The visibility of the `ExploreRevealed` LocationType (25) and the High Immigration slot was not traced.

## Q5. What player decisions does the engine ask for, and which involve uncertainty when they are made?

### Takeaway
Each decision is an `Activation` subclass (31 top-level types). Choices carry a reason enum: `SelectLocationReason` (103 values), `SelectPieceReason` (43), `SelectCardReason` (21), `SelectBoardReason` (39), `ChooseNumberReason` (14), `ChooseElementsReason` (4). The UI layer (`IntentManager`, `GameViewController`, not in the engine DLL) calls `Proceed`. The decisions with real uncertainty are growth, card plays, power targeting, and the choice of power deck. They are made before the explore card, events, fear cards, and power reveals are known.

### Cited Findings
- Base `Controller.Activation` fields: `InstanceNumber`, `_decisionMakers`, `_isOptional`, `IsExclusive`, `IsPersistent`, `SkipToAutoResolve`, `DidAutoResolve`, `NumberOfTimesUsed`, `UseActivationJournalEntry`, `InstanceIdentifier`, and others. `SelectionActivation<T>` has `_choices : List<T>`, `Selected`, `Amount`, and `AttemptAutoResolve`. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Activation types in the engine: SpiritPhase, UndoSpiritPhase, SelectGrowthGroup, UnlockGrowthGroup, PlayPowerCard, UnplayPowerCard, UsePower, MovePresence, PushPieces, GatherPieces, DealDamage, SelectLocation, SelectPiece, SelectPieces, ChoosePieceTypesForPieces, SelectCard, SelectSpirit, SelectBoard, SelectCorner, SelectEffect, ChooseElements, ChooseNumber, Continue, StartOfGame, Scenario, SpecialRule, ReminderCard, AddNewBoard, SetAdjacency, plus the `ISelectionActivation` interface and the `SelectionActivation\`1` base. — [DLL type list](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Key payloads:
  - `UsePowerActivation`: `PowerTargetChoices`, `SelectedPowerTarget`, `SourceLandChoices`, `ElementsInUse`, `ShouldSpendEnergy`, `CannotUsePowerReason`, `EffectsPreview`.
  - `PlayPowerCardActivation`: `PowerCardToPlay`, `CostModifier`.
  - `DealDamageActivation` (33 fields): `DamageDistribution`, `DamageTargetsByLocation`, `MaxDamagePerTarget`, `DamageShouldBeEfficient`.
  - `ChooseNumberActivation`: `Minimum`, `Maximum`, `PossibleChoices`, `ChooseNumberReason`.
  - `SelectEffectActivation`: ordering of triggers (`OrderingTriggerType/Timing/Priority`).

  — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- Decisions observed in the 4 logs, with counts:
  - `SelectCardActivation` 37 (`GainPower` with 4 `CardChoices`)
  - `SelectGrowthGroupActivation` 28
  - `SelectEffectActivation` 24 (trigger and effect ordering, Dahan timing)
  - `SelectLocationActivation` reasons: PowerDeckToDrawFrom 14, AddingTown 13, Push 11, RavageLand 9 (ravage order), AddingStrife 7, DealDamage 6, RemovingInvaders 3, DestroyExplorer 3, CascadingBlight 2, AddingDahan 2, AddingBeast 2, plus one each of others
  - `SelectPieceActivation` reasons: RemoveInvaderHealth 8, DestroyPiece 5, AddStrife 5, ReplacePiece 3, RemovePiece 2, DestroyPresence 2
  - `ChoosePieceTypesForPiecesActivation` 7, `ChooseElementsActivation` 5

  — [LOG-A…D](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)
- Auto-resolution: `Activation.DidAutoResolve` and `AskIf…OrderCanBeAutoResolved` (Build, Explore, Ravage) mean that many decisions with a single legal choice are taken automatically and never reach the log as a player choice. Counterattack `DealDamageActivation`s appear in `DealDamageAction` sources but not as `Proceed` lines. — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)

### Inferences
- Decisions with uncertainty at decision time (inferred from Q4's information sets):
  - Growth choice, including whether to take a gain-power growth. The 4 cards are unknown and the deck choice (minor or major) is a bet on unseen cards.
  - Which cards to play, and fast-power targeting. The explore card, events, and fear cards are unknown.
  - Fear-generation pacing. Terror-level timing interacts with unknown fear-card identities.
  - Presence placement. Future explore lands are unknown.
- Decisions with no uncertainty, given the state: cascade destination, counterattack damage distribution, presence to destroy on blight, ordering of ravage, build, and explore lands, and the Gain Power pick after the reveal. These are optimizations over a known state, and their outcomes follow the deterministic rules in Q3.
- `UseActivationJournalEntry` (`ActivationType`, `SelectionIndex`, `ChoiceIdentifiers`) in the `.dat` journals is enough to rebuild choice sets and the chosen option for behavioral modeling (inferred; not decoded).

### Gaps
- `IntentManager` and `GameViewController` live in the Unity UI assembly (not inspected). How the UI auto-proceeds is not documented here.
- Not every `SelectLocationReason` was mapped to the rule that triggers it.

## Q6. Which action and activation types appear in the logs?

### Takeaway
Across the four full `EndGame--*.txt` logs (2,431 `DidPerformAction` lines), 42 distinct action types appear, of the roughly 72 GameAction-derived classes in the engine. There are 16 distinct activation types: 12 via `IntentManager calling Proceed`, 9 via `GameViewController calling exclusive Proceed`, with 5 in both. `log.txt` is byte-identical to `EndGame--8584242708433629839.txt`.

### Cited Findings
- `DidPerformAction` type counts (4 logs combined): ChangeStateAction 276, MoveCardAction 166, MovePiecesAction 155, ChangeSpiritPhaseStatusAction 146, AdjustFearGeneratedAction 125, AddIslandPiecesAction 121, AdjustCardPlaysAction 119, GainElementsAction 97, UsePowerAction 88, ModifyPowersAction 88, DealDamageAction 76, DestroyPiecesAction 72, FlipCardAction 71, DiscardCardsAction 70, AdvanceInvaderCardsAction 69, ResolveInvaderCardAction 68, AddStatusEffectAction 63, PayEnergyAction 62, ExpireStatusEffectAction 56, LoseElementsAction 51, GainEnergyAction 50, MovePresenceAction 45, MoveCardsAction 32, RavageAction 30, ModifyMaxHealthAction 30, AddTokensAction 30, TimePassesAction 26, ResolveFearCardAction 22, ResolveEventCardAction 19, RevealCardsAction 17, AddBlightAction 17, RemoveIslandPiecesAction 13, MessageAction 11, ForgetPowerCardAction 11, CancelAction 11, SetTerrorLevelCardAction 9, DefendAction 7, GameOverAction 4, CompleteSetupAction 4, RemoveBlightAction 2, ChangeIncarnaEmpoweredAction 1, AddNewCardAction 1 (42 types). — [LOG-A…D](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)
- Activation lines:
  - `IntentManager calling Proceed`: SpiritPhase 112, UsePower 92, PlayPowerCard 68, Continue 37, StartOfGame 35, SelectGrowthGroup 28, MovePresence 26, SelectCard 21, SelectLocation 14, UnplayPowerCard 10, ChooseElements 3, UndoSpiritPhase 2.
  - `GameViewController calling exclusive Proceed`: SelectLocation 66, SelectPiece 25, SelectEffect 24, SelectCard 16, ChoosePieceTypesForPieces 7, MovePresence 5, UsePower 3, ChooseElements 2, ChooseNumber 1.

  — [LOG-A…D](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)
- Other line kinds:
  - `***** SAVING *****` 718
  - `***** CLEARING UNDO FILES *****` 117
  - `***** LOADED FILE` 31
  - "There are no damage targets to which damage can be distributed…" 26
  - "--> Invaders adjust their damage output by 1 (Dahan: x / Land: x)" 5

  — [LOG-A…D](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246831828257582.txt)
- Per-game outcomes (`GameOverAction: EndingResult=`):

  | Log | Actions | TimePasses transitions | Result |
  |---|---|---|---|
  | LOG-A | 443 | 5 | WinInvadersDestroyed |
  | LOG-B | 739 | 9 | WinInvadersDestroyed |
  | LOG-C | 675 | 7 | LoseTooMuchBlight |
  | LOG-D | 574 | 5 | WinInvadersDestroyed |

  — [LOG-A](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584242708433629839.txt); [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)

### Inferences
- About 30 engine action types never appeared, for example BuildAction, ExploreAction, RuleAction, ShuffleLocationAction, ReplacePieceAction, and CombineLandsAction. Builds and explores show up as `AddIslandPiecesAction` with `ResolveInvaderCardAction` as the source. The log is therefore not a one-to-one trace of every engine action class. A parser should key on `ResolveInvaderCardAction: … InvaderActionType: {Explore|Build|Ravage}` and on piece movements (inferred).
- A sample of 4 games (5–9 turns each) is too small to estimate outcome rates. The 45 `EndGame--*.dat` journals are the larger, structured source (Q1).

### Gaps
- Logs exist only for April 2026 games. The 41 older `EndGame--*.dat` files have no text logs, and their journal contents were not decoded.
- Not every action type's log format was checked. Some (for example `AddNewCardAction`, 1 occurrence) were seen only once.

## Q7. How does the engine scale across adversaries, expansions, and player/board count?

### Takeaway
- **Adversaries.** Several adversaries per game are supported: a list of (identifier, level) pairs, combined through a difficulty formula. The legacy single `Adversary` field is read only on deserialization.
- **Expansions.** The engine never reads expansion tags in rule code. The caller passes allowed-content lists and two flags, `useTokens` and `useEvents`. The flags switch a small number of setup rules.
- **Player and board count.** Spirit count S scales the fear-per-card threshold. Board count scales the blight pool. The invader deck size is fixed. Board count must be S or S+1, enforced by a warning only.

### Cited Findings

#### Adversaries
- **Several adversaries per game (observed).**
  - `Game::.ctor` takes `adversaryData : IEnumerable<Tuple<string, int?>>`, an (identifier, level) list.
  - `Game::CreateAdversaries` builds one `Adversary` per tuple into `_adversaries`. Each identifier is checked against `Constants.AvailableAdversaryIdentifiers`; unknown ones are skipped with the warning "`Unknown adversary identifier: … - ignoring.`".
  - The single `Game.Adversary` field is read only by `Game::Deserialize`. Every setup method (`CreateFearValues`, `CreateFearDeck`, `CreateInvaderDeck`, `CreateInvaderSlots`, `CreateStates`, `get_GameModifiers`) reads `get_Adversaries`.

  — [DLL IL + xref](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Level model (observed).**
  - `Adversary.Level : int?` plus `_adversaryLevels : List<AdversaryLevel>`.
  - `AdversaryDefinition` holds `BaseDifficulty`, `StageIIEscalationTitle/Body`, and `AdversaryLevelDefinitions`.
  - `AdversaryLevelDefinition` holds `LevelNumber`, `FearCardModifiers : int[]`, `Difficulty`, `GameEffectTitle/Body`, and `_additionalInvaderCardDefinitions`.
  - Setup hooks on `GameModifier`: `MakeSetupFearPoolAdjustments`, `MakeSetupInvaderDeckAdjustments`, `MakeSetupFearDeckAdjustments`, `MakeSetupBlightCardAdjustments`, `GetInvaderCardsRemovedBeforeRandomizing`, `TownMaxHealthAdjustment`, and `CityMaxHealthAdjustment`.
  - `AdversaryController` adds `PerformAdditionalInvaderSetup`, `Escalation`, `GetNumberOfEscalationsThatShouldOccur`, `ClearEscalationFlag`, and `AskForDamageAmountAdjustment`.
  - Level-specific play rules live in 158 types under `Controller.Adversaries` and act through the AskType overrides (Q1).

  — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Escalation (observed).** The stage-II terrain invader cards carry `"escalation": true`. User strings "`</b> will Escalate...`" and `EscalationResolvedKey` appear, and the log shows `Source: Stage II Jungle {Escalation}`. France's and Sweden's escalations consume the RNG (Q2). — [DEF/InvaderCardList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [LOG-C](file:///mnt/c/Users/brett/AppData/LocalLow/Handelabra%20Games%20Inc_/Spirit%20Island/EndGame--8584246844275351304.txt)
- **Deck changes, extra loss conditions, extra states (observed).**
  - Invader-deck edits use the definition keys `removeInvaderCard`, `moveInvaderCard`, `moveMultipleInvaderCards`, `replaceInsteadOfMove`, and `addReminderCard` (Q2).
  - Each adversary defines `additionalLossCondition` text. Examples: England's 7+ Town/City in one land; France's Town limit of 7 per starting board; Habsburg Mining's 8 Invaders/Blight in a land; Scotland's coastal-City count > 2 × boards.
  - The engine hooks are `AskForExtraLoseConditionMet` / `CheckForExtraLoseConditionMet` and `EndingResult.LoseAdversary`.
  - England's extra build step is the separate `StateType.HighImmigration`, which sits in `StateOrdering` between Fear and Ravage.

  — [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **DifficultyRating (observed).** `GameController::GetDifficultyRating` sums:
  - the scenario's `Difficulty`
  - plus the maximum adversary `Difficulty`
  - plus, when there are 2 or more adversaries, `round(0.625 × min adversary Difficulty)` (`Constants.CombinedAdversaryDifficultyFactor = 0.625`)
  - plus, when any board is thematic, `GetThematicDifficultyModifier(usingTokens)`: +1 with tokens, +3 without
  - plus 1 when the island is an archipelago
  - plus, when an extra board is present, `GetExtraBoardDifficultyModifier(priorDifficulty)`: +2, rising to +3 when prior ≥ 3 and +4 when prior ≥ 6

  The result is stored as `Game.DifficultyRating : int?`. — [DLL IL `GameController::GetDifficultyRating` and helpers](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Expansions
- **Content selection (observed).**
  - The constructor takes `useTokens`, `useEvents`, and `allowedMinor/Major/Blight/Fear/EventCardIdentifiers`, stored as the `IsUsing*` and `Allowed*` fields. The defaults are the 11 `Constants.Available*Identifiers` arrays: blight, fear, spirits, aspects, terror levels, invader, major, minor, adversaries, scenarios, events.
  - Every definition has `BaseDefinition.Expansion : string` and `RequiresTokens : IEnumerable<TokenType>`. Examples: France requires `['Strife']`, Russia requires `['Beast']`.
  - In the engine, `Expansion` is read only by the `BaseModel` and `BaseController` getters; no rule code reads it.

  — [DLL xref](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/AdversaryList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Rules switched by the flags (observed IL).**
  - `IsUsingEvents` controls whether the event deck is built and shuffled.
  - `IsUsingTokens` gates the printed setup tokens in `Game::SetupNonInvaderPiecesInLand`: `InitialBeasts`, `InitialWilds`, `InitialDiseases`, `InitialBadlands`, `InitialVitality`, and strife on the initial Explorers/Towns/Cities.
  - When `!IsUsingEvents && IsUsingTokens`, three things change:
    1. `CreateFearDeck` adds +1 to the terror-level-II fear-card count.
    2. `CreateInvaderDeck` inserts `CommandBeasts` reminder cards.
    3. `CreateBlightCard` removes `Constants.BlightCardsWithTwoBlightPerBoardOrImmediateEffects` from the blight pool, provided other cards remain.

  — [DLL IL `Model.Game::CreateFearDeck`, `::CreateInvaderDeck`, `::CreateBlightCard`, `::SetupNonInvaderPiecesInLand`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Token and piece kinds (observed).** `TokenType` covers Beast, Wilds, Disease, Strife, Scenario, Badlands, Special, and Vitality. `IslandPieceType.Thief` exists, and `Constants.DespicableTheftIdentifier` names the scenario it belongs to; the IL tying Thief to that scenario was not read. `Game` keeps a separate instance counter per kind (Q1). — [DLL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Modifier union and practice modes (observed).** `Game::get_GameModifiers` returns the Scenario (if any) plus all Adversaries. This is the set that setup hooks walk; for example, `CreateIslandPiece` sums `TownMaxHealthAdjustment` over it. `PracticeModeType` covers StartOfGameGrowth, SkipInitialInvaderActions, UnlimitedUndo, and NoBlightCard. `Game::get_UsingReducedDifficulty` is true when any practice mode is set. — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

#### Player and board count
- **Board count (observed).** The constructor warns "`View provided fewer board identifiers than spirits`" when boards < S, and "`…too many board identifiers for spirits`" when boards > S + 1. Related limits: `MaxNumberOfBoardsPerIsland = 6`, `MaxNumberOfLandsPerBoard = 16`, `IsArchipelagoAllowed`, and the `BoardCycle`/`BoardRing` types. — [DLL IL `Model.Game::.ctor`](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **What scales (observed).**
  - **Spirits S:** `FearRequiredForFearCard = 4·S`, or `5·S` when boards = S + 1, plus adversary per-player pool adjustments (England L6 `extraFearPerPlayer: 1`) (Q3).
  - **Boards:** the blight pool is `2 × InitialNumberOfBoards + 1` in normal setup. The Second Wave path (`CreateBlightCard(previousGame)`) uses `NumberOfSpirits` instead of boards. Scenario key `blightCardBlightPerBoardAdjustment` and AskType `BlightCardBlightedIslandNumberPerStartingBoardAdjustment` modify it.
  - **Fixed:** the invader deck stays 3 + 4 + 5 cards regardless of S.

  — [DLL IL](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll); [DEF/ScenarioList](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)
- **Multiple spirits (observed).**
  - The constructor takes `isGameMultiplayer : bool`.
  - It is read only through `GameController.get_IsGameMultiplayer`, which is used by `EachPlayerSelectsCardAndPerformsAction`, `EachPlayerSelectsIslandLocationAndPerformsAction`, `SelectAndDestroyPresence`, `DoesGameRequireStrictActionStructure` (per-board selections), and A Diversity of Spirits.
  - Each decision records its spirit(s) in `Activation._decisionMakers` and `StatusEffect.DecisionMakers`.
  - Energy, card plays, hands, elements, and presence stay per `Spirit`, whatever the number of humans.

  — [DLL xref](file:///mnt/c/Program%20Files%20%28x86%29/Steam/steamapps/common/Spirit%20Island/SpiritIsland_Data/Managed/SpiritIslandEngine.dll)

### Inferences
- A single human playing several spirits is modeled as `IsGameMultiplayer = false` with S `Spirit` objects. The rules scale with S and the board count, not with the number of humans. Only the "each player" prompts and the strict ordering of actions differ (inferred from the xref).
- Combined adversaries are an unordered list for difficulty purposes, since the formula uses max and min. Setup applies deck and fear adjustments over `Adversaries.Reverse()`, so list order may matter for deck edits; the leading adversary is probably first (inferred, not verified).
- For models, the useful covariates are: S; the board count (S or S+1); per-adversary (identifier, level); the scenario; `IsUsingTokens`/`IsUsingEvents`; the allowed-content pools; and `DifficultyRating` as a single summary.

### Gaps
- Not verified: how `FearCardModifiers` from two adversaries combine (summed, or only the leading one counts), and which loss conditions and escalations apply to a supporting adversary.
- The flipped-side blight count (`flippedNumberPerStartingBoard`) multiplier, and how the caller (UI) maps owned expansions to the `Allowed*` lists, were not read.
- No multi-adversary or two-board-solo game is in the four text logs, so these paths have no log confirmation.
