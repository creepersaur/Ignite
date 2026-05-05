import Prism from "prismjs";

Prism.languages.ignite = {
	comment: /\/\/.*/,
	string: {
		pattern: /\$?"(?:[^"\\]|\\.)*"/,
		greedy: true,
	},
	fstring: {
		pattern: /\$"(?:[^"\\{}]|\\.|\{[^}]*\})*"/,
		greedy: true,
		alias: "string",
	},
	keyword:
		/\b(fn|let|const|if|else|loop|while|for|in|return|break|continue|out|struct|enum|class|constructor|using|and|or|not)\b/,
	boolean: /\b(true|false|nil)\b/,

	// PascalCase = types (e.g. MyClass, Vec, String)
	"class-name": /\b[A-Z][a-zA-Z0-9]*\b/,

	// snake_case followed by ( = function call
	function:
		/\b[a-z][a-z0-9]*(?:_[a-z0-9]+)+(?=\s*\()|\b[a-z][a-z0-9]*(?=\s*\()/,

	number: /\b\d+(\.\d+)?\b/,
	operator: /[+\-*/=<>!?$]\b/,
	punctuation: /[{}[\]();,-:.]/,
};
