initSidebarItems({"fn":[["apply_save_on_load","Applies the checkpoint location if an active save warrants it."],["load_level","System that loads sprites in a given level. Can be tracked with [LevelLoadComplete]"],["unload_level","System that unloads a currently loaded level using the [UnloadLevel] tag"]],"struct":[["LevelLoadComplete","Tag that LoadLevel has completed. Added to same entity as [LoadLevel]"],["LevelLoadedSprite","Tag that entity was loaded by level, and will be removed when [UnloadLevel] instruction is given"],["LevelSaveApplied","Tag to track a level having save files applied to it."],["LoadLevel","Instruction to load a new level"],["UnloadLevel","Instruction to unload a level. Must be added to the same entity as [LoadLevel]"]]});