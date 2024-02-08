/* eslint-disable react-hooks/rules-of-hooks */
"use client";
import collider from "@/library/collider";
import AllBounties from "@/library/components/organisms/AllBounties";
import { useEffect, useRef } from "react";

const page = () => {
  const canvasParentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    collider({ canvasParentRef });
  }, []);

  return (
    <div className="w-full flex flex-col gap-8">
      <div
        className="w-full min-h-[5rem] flex flex-1 items-center justify-center"
        ref={canvasParentRef}
      ></div>
      <AllBounties />
    </div>
  );
};

export default page;
