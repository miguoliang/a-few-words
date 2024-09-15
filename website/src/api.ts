import axios from "axios";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:8000/api/v1";

interface NewWord {
  word: string;
  definition?: string;
  url?: string;
}

interface Word {
  id: number;
  word: string;
  definition?: string;
  url?: string;
  username: string;
}

interface TranslateResponse {
  text: string;
}

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export const setAuthToken = (token: string) => {
  api.defaults.headers.common["Authorization"] = `Bearer ${token}`;
};

// Updated API functions using React Query
export const useRetrieveWord = (id: number) => {
  return useQuery<Word, Error>({
    queryKey: ["word", id],
    queryFn: async () => {
      const response = await api.get(`/words/${id}`);
      return response.data;
    },
  });
};

export const useAddWord = () => {
  const queryClient = useQueryClient();
  return useMutation<Word, Error, NewWord>({
    mutationFn: async (newWord) => {
      const response = await api.post("/words", newWord);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["words"] });
    },
  });
};
export const useListWords = (offset: number = 0, limit: number = 10) => {
  return useQuery<Word[], Error>({
    queryKey: ["words", offset, limit],
    queryFn: async () => {
      const response = await api.get("/words", { params: { offset, limit } });
      return response.data;
    },
  });
};

export const useDeleteWord = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, number>({
    mutationFn: async (id: number) => {
      await api.delete(`/words/${id}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["words"] });
    },
  });
};

export const useTranslateText = () => {
  return useMutation<TranslateResponse, Error, string>({
    mutationFn: async (text) => {
      const response = await api.get("/translate", { params: { text } });
      return response.data;
    },
  });
};
