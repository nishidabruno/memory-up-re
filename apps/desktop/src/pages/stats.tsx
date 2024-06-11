import { useEffect, useState } from "react";
import {
  ResponsiveContainer,
  CartesianGrid,
  XAxis,
  YAxis,
  Tooltip,
  Area,
  AreaChart,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  Radar,
} from "recharts";

import { Sidebar } from "@components/Sidebar";
import { invoke } from "@tauri-apps/api/core";

interface Deck {
  title: string;
  id: string;
}

interface MonthlyReviews {
  count: number,
  month: string
}

interface QualityWiseReviews {
  quality: string,
  count: number
}

export const Stats = () => {
  const [decks, setDecks] = useState<Deck[]>([]);
  const [data1, setData1] = useState<MonthlyReviews[]>([]);
  const [data22, setData22] = useState<QualityWiseReviews[]>([]);

  async function loadData() {
    const decks = await invoke<Deck[]>("find_all_decks");
    setDecks(decks);
    
    const res1 = await invoke<MonthlyReviews[]>('get_total_reviews_by_month');
    setData1(res1)

    const res22 = await invoke<QualityWiseReviews[]>('get_total_reviews_by_quality');
    setData22(res22)
  }

  useEffect(() => {
    loadData();
  }, [decks]);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="h-full w-full p-4 ml-14">
        <h1 className="pb-6 pt-3 text-3xl">Estatísticas</h1>
        <h3 className="pb-2 pt-3 text-lg text-zinc-300">Reviews mensal</h3>
        <ResponsiveContainer width="100%" height="50%">
          <AreaChart
            width={500}
            height={400}
            data={data1}
            margin={{
              top: 5,
              right: 30,
              left: 20,
              bottom: 5,
            }}
          >
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="month" />
            <YAxis />
            <Tooltip
              contentStyle={{
                background: "#27272a",
                border: "none",
                borderRadius: "10px",
              }}
            />
            <Area
              type="monotone"
              dataKey="count"
              stroke="#8884d8"
              fill="#4f46e5"
            />
          </AreaChart>
        </ResponsiveContainer>
        <h3 className="pb-2 pt-3 text-lg text-zinc-300">
          Progresso de acertos
        </h3>
        <ResponsiveContainer width="100%" height="50%">
          <RadarChart cx="50%" cy="50%" outerRadius="80%" data={data22}>
            <PolarGrid />
            <PolarAngleAxis dataKey="quality" spacing={10} />
            <Radar
              dataKey="count"
              stroke="#8884d8"
              fill="#4f46e5"
              fillOpacity={0.6}
            />
          </RadarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};
