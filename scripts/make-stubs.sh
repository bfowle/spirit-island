#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

stub() {
  local path="$1" title="$2" milestone="$3"
  [ -f "src/$path" ] && return 0
  mkdir -p "$(dirname "src/$path")"
  cat >"src/$path" <<EOF
# $title

\`\`\`admonish note "Stub"
Coming in **${milestone}**. See the [SUMMARY](../SUMMARY.md) for chapters currently available.
\`\`\`
EOF
}

# Root
stub "introduction.md" "Introduction" "M1"
stub "how-to-use.md" "How to Use This Book" "M1"
stub "reader-self-assessment.md" "Reader Self-Assessment" "M1"

# Foundations
stub "foundations/core-loop.md" "The Core Loop" "M1"
stub "foundations/terminology.md" "Terminology" "M1"
stub "foundations/reading-a-card.md" "Reading a Card" "M1"

# Fundamentals
stub "fundamentals/terrain.md" "Terrain" "M1"
stub "fundamentals/adjacency-and-range.md" "Adjacency & Range" "M2"
stub "fundamentals/tempo.md" "Tempo" "M1"
stub "fundamentals/presence-economy.md" "Presence Economy" "M1"
stub "fundamentals/major-vs-minor.md" "Major vs. Minor — A Stats-Backed Framework" "M1"
stub "fundamentals/energy-curves.md" "Energy Curves" "M2"
stub "fundamentals/card-draft-theory.md" "Card Draft Theory" "M2"
stub "fundamentals/fear-track.md" "The Fear Track" "M1"
stub "fundamentals/blight-track.md" "The Blight Track" "M2"
stub "fundamentals/dahan.md" "Dahan — The Island's People" "M1"

# Spirits
stub "spirits/index.md" "Spirits — Overview & Index" "M1"
stub "spirits/aspects.md" "Aspects" "M4"

# Low complexity
stub "spirits/low/river-surges-in-sunlight.md" "River Surges in Sunlight" "M2"
stub "spirits/low/lightning-swift-strike.md" "Lightning's Swift Strike" "M2"
stub "spirits/low/shadows-flicker-like-flame.md" "Shadows Flicker Like Flame" "M1"
stub "spirits/low/vital-strength-of-the-earth.md" "Vital Strength of the Earth" "M2"
stub "spirits/low/ocean-hungry-grasp.md" "Ocean's Hungry Grasp" "M1"
stub "spirits/low/a-spread-of-rampant-green.md" "A Spread of Rampant Green" "M1"
stub "spirits/low/thunderspeaker.md" "Thunderspeaker" "M1"
stub "spirits/low/bringer-of-dreams-and-nightmares.md" "Bringer of Dreams and Nightmares" "M1"

# Moderate
stub "spirits/moderate/sharp-fangs-behind-the-leaves.md" "Sharp Fangs Behind the Leaves" "M2"
stub "spirits/moderate/keeper-of-the-forbidden-wilds.md" "Keeper of the Forbidden Wilds" "M2"
stub "spirits/moderate/heart-of-the-wildfire.md" "Heart of the Wildfire" "M3"
stub "spirits/moderate/serpent-slumbering-beneath-the-island.md" "Serpent Slumbering Beneath the Island" "M2"
stub "spirits/moderate/downpour-drenches-the-world.md" "Downpour Drenches the World" "M2"
stub "spirits/moderate/finder-of-paths-unseen.md" "Finder of Paths Unseen" "M2"
stub "spirits/moderate/devouring-teeth-lurk-underfoot.md" "Devouring Teeth Lurk Underfoot" "M4"
stub "spirits/moderate/eyes-watch-from-the-trees.md" "Eyes Watch from the Trees" "M4"
stub "spirits/moderate/fathomless-mud-of-the-swamp.md" "Fathomless Mud of the Swamp" "M4"
stub "spirits/moderate/rising-heat-of-stone-and-sand.md" "Rising Heat of Stone and Sand" "M4"
stub "spirits/moderate/sun-bright-whirlwind.md" "Sun-Bright Whirlwind" "M4"
stub "spirits/moderate/shifting-memory-of-ages.md" "Shifting Memory of Ages" "M3"
stub "spirits/moderate/grinning-trickster.md" "Grinning Trickster Stirs Up Trouble" "M3"
stub "spirits/moderate/many-minds-move-as-one.md" "Many Minds Move as One" "M3"
stub "spirits/moderate/shroud-of-silent-mist.md" "Shroud of Silent Mist" "M3"
stub "spirits/moderate/vengeance-burning-plague.md" "Vengeance as a Burning Plague" "M3"
stub "spirits/moderate/volcano-looming-high.md" "Volcano Looming High" "M3"

# High
stub "spirits/high/stone-unyielding-defiance.md" "Stone's Unyielding Defiance" "M3"
stub "spirits/high/lure-of-the-deep-wilderness.md" "Lure of the Deep Wilderness" "M3"
stub "spirits/high/fractured-days-split-the-sky.md" "Fractured Days Split the Sky" "M3"
stub "spirits/high/starlight-seeks-its-form.md" "Starlight Seeks Its Form" "M3"
stub "spirits/high/wounded-waters-bleeding.md" "Wounded Waters Bleeding" "M4"
stub "spirits/high/dances-up-earthquakes.md" "Dances Up Earthquakes" "M4"
stub "spirits/high/ember-eyed-behemoth.md" "Ember-Eyed Behemoth" "M4"
stub "spirits/high/relentless-gaze-of-the-sun.md" "Relentless Gaze of the Sun" "M4"
stub "spirits/high/towering-roots-of-the-jungle.md" "Towering Roots of the Jungle" "M4"
stub "spirits/high/hearth-vigil.md" "Hearth-Vigil" "M4"
stub "spirits/high/breath-of-darkness.md" "Breath of Darkness Down Your Spine" "M4"

# Very high
stub "spirits/very-high/wandering-voice.md" "Wandering Voice Keens Delirium" "M4"
stub "spirits/very-high/covets-gleaming-shards.md" "Covets Gleaming Shards of Earth" "M5"
stub "spirits/very-high/ferocious-warrior.md" "Ferocious Warrior of the Lost Lands" "M5"

# Adversaries
stub "adversaries/index.md" "Adversaries — Overview & Index" "M1"
stub "adversaries/england.md" "England" "M1"
stub "adversaries/brandenburg-prussia.md" "Brandenburg-Prussia" "M1"
stub "adversaries/sweden.md" "Sweden" "M1"
stub "adversaries/france-plantation-colony.md" "France (Plantation Colony)" "M2"
stub "adversaries/habsburg-mining-expedition.md" "Habsburg Mining Expedition" "M2"
stub "adversaries/russia.md" "Russia" "M2"
stub "adversaries/scotland.md" "Scotland" "M2"
stub "adversaries/habsburg-livestock-colony.md" "Habsburg Livestock Colony" "M2"

# Scenarios
stub "scenarios/index.md" "Scenarios — Overview & Index" "M2"
for s in \
  "blitz:Blitz:M2" \
  "guard-the-isles-heart:Guard the Isle's Heart:M2" \
  "rituals-of-terror:Rituals of Terror:M2" \
  "dahan-insurrection:Dahan Insurrection:M2" \
  "second-wave:Second Wave:M2" \
  "powers-long-forgotten:Powers Long Forgotten:M2" \
  "ward-the-shores:Ward the Shores:M2" \
  "rituals-of-destroying-flame:Rituals of the Destroying Flame:M2" \
  "despicable-theft:Despicable Theft:M3" \
  "elemental-invocation:Elemental Invocation:M3" \
  "a-diversity-of-spirits:A Diversity of Spirits:M3" \
  "the-great-river:The Great River:M3" \
  "varied-terrains:Varied Terrains:M3" \
  "destiny-unfolds:Destiny Unfolds:M4" \
; do
  IFS=':' read -r slug title ms <<< "$s"
  stub "scenarios/$slug.md" "$title" "$ms"
done

# Combos
stub "combos/archetype-index.md" "Archetype Index" "M3"
stub "combos/energy-denial.md" "Energy Denial" "M3"
stub "combos/dahan-rush.md" "Dahan Rush" "M3"
stub "combos/fear-rush.md" "Fear Rush" "M3"
stub "combos/terrain-control.md" "Terrain Control" "M3"
stub "combos/defend-and-outlast.md" "Defend & Outlast" "M3"
stub "combos/major-power-shopping.md" "Major Power Shopping" "M3"

# Social
stub "social/index.md" "Playing with Others — Overview" "M1"
stub "social/teaching-methods.md" "Teaching Methods — Hook, Quick, Full, Layered" "M1"
stub "social/two-player.md" "Two-Player Dynamics" "M4"
stub "social/three-and-four-player.md" "Three & Four Player Dynamics" "M4"
stub "social/alpha-player-problem.md" "The Alpha-Player Problem" "M1"
stub "social/mixed-skill-tables.md" "Mixed-Skill Tables" "M4"
stub "social/convention-and-meetup-norms.md" "Convention & Meetup Norms" "M4"
stub "social/playing-with-strangers.md" "Playing with Strangers" "M4"
stub "social/post-game-debrief.md" "The Post-Game Debrief" "M4"
stub "social/teaching-anti-patterns.md" "Teaching Anti-Patterns" "M4"

# Statistics
stub "statistics/index.md" "Statistics — Why and How" "M4"
stub "statistics/reading-mindwanderer.md" "Reading mindwanderer" "M4"
stub "statistics/digital-vs-tabletop.md" "Digital vs. Tabletop Data" "M4"
stub "statistics/fear-card-expected-value.md" "Fear Card Expected Value" "M4"
stub "statistics/event-deck-risk-profiles.md" "Event Deck Risk Profiles" "M4"
stub "statistics/aspect-power-deltas.md" "Aspect Power Deltas" "M4"
stub "statistics/major-vs-minor-rates.md" "Major vs. Minor Draft Rates" "M4"
stub "statistics/self-tracking.md" "Self-Tracking Your Games" "M4"

# Appendices
stub "appendices/card-reference.md" "Curated Card Reference" "M5"
stub "appendices/fear-blight-event-decks.md" "Fear, Blight & Event Decks Reference" "M5"
stub "appendices/glossary.md" "Glossary" "M1"
stub "appendices/sources.md" "Sources & Citations" "M1"
stub "appendices/changelog-of-meta.md" "Meta Changelog" "M1"
stub "appendices/if-youve-only-played.md" "If You've Only Played..." "M6"
stub "appendices/claude-skills-guide.md" "Claude Skills Companion" "M1"

echo "STUBS_DONE"
