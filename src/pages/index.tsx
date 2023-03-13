import type { NextPage } from "next";
import Head from "next/head";
import { SolanaTweeterView } from "../views";

const Home: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Solana Ekşi</title>
        <meta name="description"/>
      </Head>
      <SolanaTweeterView />
    </div>
  );
};

export default Home;
