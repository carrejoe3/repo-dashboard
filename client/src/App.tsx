import { useState } from "react";
import "./App.css";

import ResultsTable from "./components/ResultsTable";
import Button from "./components/Button";
import { ResultsText } from "./types";

function App() {
  const [owner, setOwner] = useState("carrejoe3");
  const [repoName, setRepoName] = useState("wedding-site");
  const [buttonDisabled, setButtonDisabled] = useState(false);
  const [resultsText, setResultsText] = useState<ResultsText>(
    "Results will be shown here...",
  );
  const buttonsDisabled = buttonDisabled || owner === "" || repoName === "";

  const fetchRepoData = async (route: string) => {
    try {
      setButtonDisabled(true);
      const response = await fetch(
        `http://localhost:3030/${route}/${owner}/${repoName}`,
      );

      if (!response.ok) {
        setButtonDisabled(false);
        setResultsText("Error fetching repo");
        throw new Error(`Error: ${response.statusText}`);
      }

      console.log("Response:", response);
      const data = await response.json();

      console.log("Data:", data);

      setButtonDisabled(false);
      setResultsText(data);
    } catch (error) {
      setButtonDisabled(false);
      console.error("Error fetching repo:", error);
      setResultsText("Error fetching repo");
    }
  };

  return (
    <div className="bg-gray-100 flex flex-col items-center p-4 w-full">
      <h1 className="text-3xl font-bold text-gray-800 mb-6">Repo Search</h1>
      <div className="text-gray-800">Owner</div>
      <div className="w-full max-w-md">
        <input
          type="text"
          placeholder="Search..."
          value={owner}
          onChange={(e) => setOwner(e.target.value)}
          className="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800"
        />
      </div>
      <div className="text-gray-800 pt-5">Repo Name</div>
      <div className="w-full max-w-md">
        <input
          type="text"
          placeholder="Search..."
          value={repoName}
          onChange={(e) => setRepoName(e.target.value)}
          className="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800"
        />
      </div>
      <div className="w-full flex justify-center">
        <Button
          onClick={() => fetchRepoData("dep_tree")}
          disabled={buttonsDisabled}
        >
          Check Outdated
        </Button>
        <Button
          onClick={() => fetchRepoData("dep_tree")}
          disabled={buttonsDisabled}
        >
          Get Dependency Tree
        </Button>
      </div>
      <ResultsTable resultsText={resultsText} />
    </div>
  );
}

export default App;
