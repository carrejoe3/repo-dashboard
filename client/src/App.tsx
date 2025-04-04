import { useState } from "react";
import "./App.css";

import ResultsTable from "./components/ResultsTable";
import Button from "./components/Button";
import ForceGraph3D from "react-force-graph-3d";

import { useFetchOutdated } from "./hooks/useFetchOutdated";
import { useFetchDepTree } from "./hooks/useFetchDepTree";

function App() {
  const [owner, setOwner] = useState("carrejoe3");
  const [repoName, setRepoName] = useState("wedding-site");
  const {
    data: outdatedData,
    loading: fetchOutdatedloading,
    error: fetchOutdatederror,
    fetchOutdated,
  } = useFetchOutdated();
  const {
    data: depsData,
    loading: fetchDepsLoading,
    error: fetchDepsError,
    fetchDepTree,
  } = useFetchDepTree();

  const buttonsDisabled =
    fetchOutdatedloading || fetchDepsLoading || !owner || !repoName;

  const handleFetchOutdated = () => {
    fetchOutdated(owner, repoName);
  };

  const handleFetchDepTree = () => {
    fetchDepTree(owner, repoName);
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
          onClick={() => handleFetchOutdated()}
          disabled={buttonsDisabled}
        >
          Check Outdated
        </Button>
        <Button onClick={() => handleFetchDepTree()} disabled={buttonsDisabled}>
          Get Dependency Tree
        </Button>
      </div>
      {fetchOutdatederror && (
        <p className="text-red-500">{fetchOutdatederror}</p>
      )}
      {fetchDepsError && <p className="text-red-500">{fetchDepsError}</p>}
      <ResultsTable resultsText={outdatedData} />
      {depsData && <ForceGraph3D graphData={depsData} />}
    </div>
  );
}

export default App;
