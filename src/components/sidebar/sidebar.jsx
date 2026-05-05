import { Link } from "react-router-dom";
import "./sidebar.css";
import { getDocumentation } from "../../docs/documentation";
import { useParams } from "react-router-dom";

function DocButton({ link, active, children }) {
	return (
		<Link
			className="doc-button"
			to={link}
			active={active && "true" || "false"}
		>
			{children || "Home Button"}
		</Link>
	);
}

function DocSection({ name, children }) {
	return (
		<div className="doc-section">
			<p>{name || "Hello"}</p>
			{children}
		</div>
	);
}

function map_docs(docs, docPath) {
	return docs.map((item) => {
		if (item.pages) {
			return (
				<DocSection name={item.name} key={item.name}>
					{map_docs(item.pages, docPath)}
				</DocSection>
			);
		} else {
			const subLink = item.path.split(".")[0];
			return (
				<DocButton
					link={"/docs/" + subLink}
					active={subLink == docPath}
					key={item.path}
				>
					{item.name}
				</DocButton>
			);
		}
	});
}

export default function Sidebar() {
	const { "*": docPath } = useParams();
	let docs = getDocumentation();

	return (
		<div className="sidebar">
			{map_docs(docs, docPath)}
		</div>
	);
}
