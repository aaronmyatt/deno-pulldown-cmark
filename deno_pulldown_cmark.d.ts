/* tslint:disable */
/* eslint-disable */
/**
* @param {string} contents
* @returns {Array<{ type: String, tag: String, content: String, level: Number, kind: String, fenced: Boolean, language: String, start_number: Number, label: String, alignments: Array<String>, url: String, title: String, checked: Boolean }>}
*/
export function parse(contents: string): Array<{
    type: String,
    tag: String,
    content: String,
    level: Number,
    kind: String,
    fenced: Boolean,
    language: String,
    start_number: Number,
    label: String,
    alignments: Array<String>,
    url: String,
    title: String,
    checked: Boolean
}>;
/**
* @param {string} contents
* @returns {string}
*/
export function html(contents: string): string;
