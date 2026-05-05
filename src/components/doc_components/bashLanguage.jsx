import Prism from "prismjs";

Prism.languages.bash = {
	comment: /#.*/,
	string: {
		pattern: /\$?"(?:[^"\\]|\\.)*"/,
		greedy: true,
	},
    command: {
        pattern: /\w*/,
        lookbehind: true,
        alias: "boolean",
    },
	number: /\b\d+(\.\d+)?\b/,
};
