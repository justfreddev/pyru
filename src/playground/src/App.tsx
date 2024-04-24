import { useState } from "react";

import CodeEditor from "./components/editor/CodeEditor";
import Navbar from "./components/navbar/Navbar";
import SyntaxView from "./components/syntaxview/SyntaxView";

import "./App.css";

function App() {
    const [isViewingSyntax, setIsViewingSyntax] = useState(false);

    function toggleViewingSyntax() {
        setIsViewingSyntax((prev) => !prev);
    }

    return (
        <div>
            <Navbar toggleViewingSyntax={toggleViewingSyntax} />
            <CodeEditor />

            {isViewingSyntax && (
                <SyntaxView toggleViewingSyntax={toggleViewingSyntax} />
            )}
        </div>
    );
}

export default App;
