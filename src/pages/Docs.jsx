import { useParams } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import Sidebar from "../components/sidebar/sidebar";
import TopBar from "../components/topbar/topbar";
import ReactMarkdown from "react-markdown";
import rehypeRaw from "rehype-raw";
import "./styles/docs.css";
import "../components/doc_components/doc_component.css";
import Overview from "../components/overview/overview";
import "../components/doc_components/code_block";
import CodeBlock from "../components/doc_components/code_block";

export default function Docs() {
	const { "*": docPath } = useParams();
	const [content, setContent] = useState("← Select a doc from the left");
	const [headings, setHeadings] = useState([]);
	const contentRef = useRef();

	useEffect(() => {
		if (docPath.length > 0) {
			fetch(`/ignite/docs/${docPath}.md`)
				.then((res) => res.text())
				.then(setContent)
				.catch(() => setContent("# Page not found"));
		}
	}, [docPath]);

	useEffect(() => {
		if (!contentRef.current) return;

		const nodes = contentRef.current.querySelectorAll("h1, h2, h3");
		const collected = Array.from(nodes).map((el) => {
			const id = el.textContent.toLowerCase().replace(/\s+/g, "-");
			el.id = id;
			return {
				text: el.textContent,
				level: Number(el.tagName[1]),
				element: el,
				id,
			};
		});
		setHeadings(collected);
	}, [content]);

	return (
		<>
			<TopBar />

			<div className="docs-main">
				<Sidebar />
				<div className="doc-content" ref={contentRef}>
					<ReactMarkdown
						rehypePlugins={[rehypeRaw]}
						components={{
							code({ inline, className, children }) {
								const match = /language-(\w+)/.exec(
									className || "",
								);
								return !inline && match
									? (
										<CodeBlock
											code={String(children).trim()}
											language={match[1]}
										/>
									)
									: (
										<code className={className}>
											{children}
										</code>
									);
							},
						}}
					>
						{content}
					</ReactMarkdown>
				</div>
				<Overview headings={headings} />
			</div>
		</>
	);
}
