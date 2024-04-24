import "./SyntaxView.css";

type SyntaxViewProps = {
    toggleViewingSyntax: () => void;
};

const SyntaxView: React.FC<SyntaxViewProps> = ({ toggleViewingSyntax }) => {
    return (
        <div className="syntax-view">
            <div className="syntax-view-header">
                <h2>Syntax</h2>
                <a className="syntax-view-cross" onClick={toggleViewingSyntax}>
                    <svg
                        width="46"
                        height="46"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="round"
                        strokeLinejoin="round"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path d="M18 6 6 18"></path>
                        <path d="m6 6 12 12"></path>
                    </svg>
                </a>
            </div>
            <p>Here is the syntax view.</p>
        </div>
    );
};

export default SyntaxView;
