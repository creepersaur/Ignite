import Prism from "prismjs";
import { Copy } from "lucide-react";
import "./code.css";
import "./bashLanguage"
import "./igniteLanguage"

export default function CodeBlock({ code, language }) {
	const grammar = Prism.languages[language] ?? Prism.languages.markup;

	const lines = code.split("\n").map((line, i) => {
		const html = Prism.highlight(line, grammar, language);
		const replaced = html.replaceAll("\t", '<span class="token tab">\t</span>');

		return (
			<div key={i} className="code-line">
				<span className="line-number">{i + 1}</span>
				<span dangerouslySetInnerHTML={{ __html: replaced }} />
			</div>
		);
	});

	return (
		<pre className={`language-${language}`} data-language={language}>
		<button className="copy-btn" onClick={() => navigator.clipboard.writeText(code)}>
			<Copy size={16} color="#5e5e5e"/>
		</button>
		{lines}
		</pre>
	);
}
