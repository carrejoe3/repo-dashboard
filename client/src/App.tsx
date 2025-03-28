import { useState } from 'react'
import './App.css'

type ResultsText = string | {
  [packageName: string]: {
    current: string;
    wanted: string;
    latest: string;
  };
}

function App() {
  const [owner, setOwner] = useState('carrejoe3')
  const [repoName, setRepoName] = useState('wedding-site')
  const [buttonDisabled, setButtonDisabled] = useState(false)
  const [resultsText, setResultsText] = useState<ResultsText>('Results will be shown here...')

  const fetchRepoData = async () => {
    try {
      setButtonDisabled(true);

      const response = await fetch(`http://localhost:3030/outdated/${owner}/${repoName}`);

      if (!response.ok) {
        setResultsText('Error fetching repo');
        throw new Error(`Error: ${response.statusText}`);
      }

      const data = await response.json();

      setResultsText(JSON.parse(data))
    } catch (error) {
      console.error('Error fetching repo:', error);
      setResultsText('Error fetching repo');
    }
  };

  return (
    <div className="bg-gray-100 flex flex-col items-center p-4 w-full">
      <h1 className="text-3xl font-bold text-gray-800 mb-6">Repo Search</h1>
      <div className='text-gray-800'>Owner</div>
      <div className="w-full max-w-md">
        <input
          type="text"
          placeholder="Search..."
          value={owner}
          onChange={(e) => setOwner(e.target.value)}
          className="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800"
        />
      </div>
      <div className='text-gray-800 pt-5'>Repo Name</div>
      <div className="w-full max-w-md">
        <input
          type="text"
          placeholder="Search..."
          value={repoName}
          onChange={(e) => setRepoName(e.target.value)}
          className="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800"
        />
      </div>
      <div className='p-3 pt-5 w-full'>
        <button
          onClick={() => fetchRepoData()}
          className='shadow-sm'
          disabled={buttonDisabled || owner === '' || repoName === ''}>
          Search
        </button>
      </div>
      <div className='p-3 w-full text-gray-800'>
        {(typeof resultsText === 'object') ? (
          <table className="table-auto border-collapse border border-gray-400 w-full text-gray-800">
            <thead>
              <tr>
                <th className="border border-gray-400 px-4 py-2">Package</th>
                <th className="border border-gray-400 px-4 py-2">Current</th>
                <th className="border border-gray-400 px-4 py-2">Wanted</th>
                <th className="border border-gray-400 px-4 py-2">Latest</th>
              </tr>
            </thead>
            <tbody>
              {Object.entries(resultsText).map(([packageName, details]) => (
                <tr key={packageName}>
                  <td className="border border-gray-400 px-4 py-2">{packageName}</td>
                  <td className="border border-gray-400 px-4 py-2">{details.current}</td>
                  <td className="border border-gray-400 px-4 py-2">{details.wanted}</td>
                  <td className="border border-gray-400 px-4 py-2">{details.latest}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : resultsText}
      </div>
    </div>
  )
}

export default App
