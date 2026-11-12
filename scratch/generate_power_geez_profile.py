#!/usr/bin/env python3
"""
Convert verified SIL Ethiopic Power-G .kmn source to Fidel Amharic profile TOML.
Preserves MIT license and author attribution.
"""
import re
import sys

def parse_kmn(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    def extract_store(name):
        pattern = rf'store\s*\(\s*{name}\s*\)\s+((?:[^\n]+\\\s*\n)*[^\n]+)'
        m = re.search(pattern, content)
        if not m:
            return []
        raw = m.group(1).replace('\\\n', ' ').replace('\n', ' ')
        items = []
        for tok in raw.split():
            tok = tok.strip('"').strip("'")
            if tok.startswith('U+') or tok.startswith('u+'):
                items.append(chr(int(tok[2:], 16)))
            elif tok:
                items.append(tok)
        return items

    keys = extract_store("key")
    first = extract_store("first")
    second = extract_store("second")
    third = extract_store("third")
    fourth = extract_store("fourth")
    fifth = extract_store("fifth")
    sixth = extract_store("sixth")
    seventh = extract_store("seventh")
    eighth = extract_store("eighth")
    diph = extract_store("diph")

    rules = []
    seen = set()

    def add_rule(k, out, order=None):
        if not k or not out or k in seen:
            return
        seen.add(k)
        rules.append((k, out, order))

    # Single standalone vowels
    vowels = [
        ('u', '\u12A1', 2),
        ('i', '\u12A2', 3),
        ('a', '\u12A3', 4),
        ('e', '\u12A5', 6),
        ('o', '\u12A6', 7),
        ('W', '\u12A7', 1),
        ('U', '\u12D1', 2),
        ('I', '\u12D2', 3),
        ('A', '\u12D3', 4),
        ('E', '\u12D5', 6),
        ('O', '\u12D6', 7),
    ]
    for k, out, o in vowels:
        add_rule(k, out, o)

    # Base consonants and vowel forms
    for i in range(min(len(keys), len(first))):
        k = keys[i]
        f1 = first[i]
        add_rule(k, f1, 1)

        if i < len(second): add_rule(k + 'u', second[i], 2)
        if i < len(third):  add_rule(k + 'i', third[i], 3)
        if i < len(fourth): add_rule(k + 'a', fourth[i], 4)
        if i < len(fifth):  add_rule(k + 'y', fifth[i], 5)
        if i < len(sixth):  add_rule(k + 'e', sixth[i], 6)
        if i < len(seventh):add_rule(k + 'o', seventh[i], 7)
        if i < len(eighth) and eighth[i] != f1:
            add_rule(k + 'O', eighth[i], 8)
        if i < len(diph) and diph[i] != f1:
            add_rule(k + 'W', diph[i], 4)

    # Multi-letter aliases common in phonetic typing
    aliases = [
        ('sh', 'S'),
        ('ch', 'c'),
        ('ny', 'N'),
        ('zh', 'Z'),
        ('ts', 'F'),
        ('kh', 'K'),
    ]
    for alias, base_key in aliases:
        if base_key in keys:
            idx = keys.index(base_key)
            f1 = first[idx]
            add_rule(alias, f1, 1)
            if idx < len(second): add_rule(alias + 'u', second[idx], 2)
            if idx < len(third):  add_rule(alias + 'i', third[idx], 3)
            if idx < len(fourth): add_rule(alias + 'a', fourth[idx], 4)
            if idx < len(fifth):  add_rule(alias + 'y', fifth[idx], 5)
            if idx < len(sixth):  add_rule(alias + 'e', sixth[idx], 6)
            if idx < len(seventh):add_rule(alias + 'o', seventh[idx], 7)
            if idx < len(diph) and diph[idx] != f1:
                add_rule(alias + 'W', diph[idx], 4)
                add_rule(alias + 'wa', diph[idx], 4)

    # Alternative forms with '['
    add_rule("h[", "\u1280", 1) # ኀ
    add_rule("H[", "\u12B8", 1) # ኸ
    add_rule("s[", "\u1220", 1) # ሠ
    add_rule("t[", "\u1338", 1) # ጸ
    add_rule("T[", "\u1340", 1) # ፀ

    # Punctuation
    add_rule(":", "\u1361", None)   # ፡
    add_rule("::", "\u1368", None)  # ፨
    add_rule(".", "\u1362", None)   # ።
    add_rule(",", "\u1365", None)   # ፥
    add_rule(",,", "\u1363", None)  # ፣
    add_rule(";", "\u1364", None)   # ፤
    add_rule(":-", "\u1366", None)  # ፦
    add_rule(":.", "\u1367", None)  # ፧

    # Ethiopic numbers
    numbers = [
        ('1*', '\u1369'), ('2*', '\u136A'), ('3*', '\u136B'), ('4*', '\u136C'),
        ('5*', '\u136D'), ('6*', '\u136E'), ('7*', '\u136F'), ('8*', '\u1370'),
        ('9*', '\u1371'), ('10*', '\u1372'), ('20*', '\u1373'), ('30*', '\u1374'),
        ('40*', '\u1375'), ('50*', '\u1376'), ('60*', '\u1377'), ('70*', '\u1378'),
        ('80*', '\u1379'), ('90*', '\u137A'), ('100*', '\u137B'), ('10000*', '\u137C')
    ]
    for k, out in numbers:
        add_rule(k, out, None)

    return rules

def main():
    rules = parse_kmn('/tmp/sil_power_g.kmn')
    print(f"Generated {len(rules)} mapping rules.")

    toml_out = '''[metadata]
id = "power_geez_phonetic_unicode"
display_name = "Power Geez Phonetic-compatible (Unicode)"
language = "am"
script = "Ethi"
version = "1.2.6"
license = "MIT"
author = "SIL Ethiopia (adapted for Fidel Amharic)"
description = "Phonetic keyboard sequences compatible with the Power Geez (Phonetic) layout, outputting standard Unicode Ethiopic script."

[options]
case_sensitive = true
terminators = ["'"]
auto_commit_timeout_ms = 0

'''
    for k, out, o in rules:
        escaped_k = k.replace('\\', '\\\\').replace('"', '\\"')
        escaped_out = out.replace('\\', '\\\\').replace('"', '\\"')
        order_str = f"\norder = {o}" if o is not None else ""
        toml_out += f'[[rules]]\nkeys = "{escaped_k}"\noutput = "{escaped_out}"{order_str}\n\n'

    with open("/home/abdu/fidel/profiles/power_geez_phonetic_unicode.toml", "w", encoding="utf-8") as f:
        f.write(toml_out)
    print("Wrote profiles/power_geez_phonetic_unicode.toml successfully.")

if __name__ == "__main__":
    main()
