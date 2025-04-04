import { useState } from "react";
import { ResultsText } from "../types";

export const useFetchOutdated = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [data, setData] = useState<ResultsText>(
    "Results will be shown here...",
  )

  const fetchOutdated = async (owner: string, repoName: string) => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch(
        `http://localhost:3030/outdated/${owner}/${repoName}`
      );

      if (!response.ok) {
        throw new Error(`Error: ${response.statusText}`);
      }

      const result = await response.json();
      setData(JSON.parse(result));
    } catch (err) {
      setError(err instanceof Error ? err.message : "Error fetching data");
    } finally {
      setLoading(false);
    }
  };

  return { data, loading, error, fetchOutdated };
};
