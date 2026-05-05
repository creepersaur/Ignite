import "./overview.css";

export default function Overview({ headings }) {
	return (
		<>
			<div className="overview">
				<h3>On This Page...</h3>
				<div className="overview-buttons">
					{headings.map((item) => {
						return (
							<button
								data-level={item.level}
								onClick={() =>
									item.element.scrollIntoView({
										behavior: "smooth",
										block: "start",
									})}
							>
								{item.text}
							</button>
						);
					})}
				</div>
			</div>
		</>
	);
}
