/**
 * Fidel Amharic C ABI Header
 * Copyright (c) Fidel Amharic Project (MIT License)
 */

#ifndef FIDEL_H
#define FIDEL_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    FIDEL_ACTION_PASS_THROUGH = 0,
    FIDEL_ACTION_CONSUMED = 1,
    FIDEL_ACTION_COMMIT = 2,
    FIDEL_ACTION_SET_COMPOSITION = 3,
    FIDEL_ACTION_COMMIT_AND_COMPOSE = 4,
    FIDEL_ACTION_CLEAR_COMPOSITION = 5,
} FidelActionType;

typedef struct {
    FidelActionType action_type;
    const char* text;      /* Commit text (valid until next engine call) */
    const char* display;   /* Composition display text */
    const char* raw;       /* Composition raw ASCII */
} CFidelAction;

typedef struct FidelEngineHandle FidelEngineHandle;

/* Engine lifecycle */
FidelEngineHandle* fidel_engine_create(const char* profile_toml);
void fidel_engine_destroy(FidelEngineHandle* handle);
void fidel_engine_reset(FidelEngineHandle* handle);

/* Key processing */
int fidel_engine_process_char(FidelEngineHandle* handle, uint32_t unicode_char, CFidelAction* out_action);
int fidel_engine_process_special(FidelEngineHandle* handle, int special_code, CFidelAction* out_action);

/* Info */
const char* fidel_engine_version(void);

#ifdef __cplusplus
}
#endif

#endif /* FIDEL_H */
