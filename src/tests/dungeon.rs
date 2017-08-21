use super::super::*;

const monster_pool: Vec<Monster> = vec![Monster::new()];
const template_monster_pool: Vec<TemplateMonster> = vec![Monster::new()];
const theme_keyword_pool: Vec<Keyword> = vec![];

// TODO: verify that dungeons generated with a specific seed are always identical
// TODO: verify that dungeons respect their invariant arguments
/*
 * TODO: verify that sub-dungeons always only contain theme keywords of their
 * super-dungeons
 */
