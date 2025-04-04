import { ResultsText } from "../types";

export default function ResultsTable({
  resultsText,
}: {
  resultsText: ResultsText;
}) {
  return (
    <div className="p-3 w-full text-gray-800">
      {typeof resultsText === "object" ? (
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
                <td className="border border-gray-400 px-4 py-2">
                  {packageName}
                </td>
                <td className="border border-gray-400 px-4 py-2">
                  {details.current}
                </td>
                <td className="border border-gray-400 px-4 py-2">
                  {details.wanted}
                </td>
                <td className="border border-gray-400 px-4 py-2">
                  {details.latest}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      ) : (
        resultsText
      )}
    </div>
  );
}
