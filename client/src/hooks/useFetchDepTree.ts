import { useState } from "react";

export const useFetchDepTree = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [data, setData] = useState<string>(
    "Results will be shown here...",
  );

  const fetchDepTree = async (owner: string, repoName: string) => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch(
        `http://localhost:3030/dep_tree/${owner}/${repoName}`,
      );

      if (!response.ok) {
        throw new Error(`Error: ${response.statusText}`);
      }

      const result = await response.text();
      setData(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Error fetching data");
    } finally {
      setLoading(false);
    }
  };

  return { data, loading, error, fetchDepTree };
};
