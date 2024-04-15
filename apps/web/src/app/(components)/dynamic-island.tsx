import { useRef } from "react";
import { create } from "zustand";

import { useAutoAnimate } from "@formkit/auto-animate/react";

type DynamicIslandState = {
    orgName: string;
    setOrgName: (orgName: string) => void;
};
const useStore = create<DynamicIslandState>((set) => ({
    orgName: "verigoers",
    setOrgName: (orgName: string) => set(() => ({ orgName })),
}));

export function DynamicIsland() {
    const orgName = useStore();

    const divRef = useRef<HTMLDivElement>(null);
    const [parent] = useAutoAnimate({
        duration: 500,
        disrespectUserMotionPreference: true,
    });
    return (
        <div>
            <div
                onClick={() => {
                    orgName.setOrgName("largeeeeeeeaaaaaaaaa");
                }}
                className="rounded-full bg-black p-12 py-2 text-white shadow-xl"
                id="dyamicIsland"
            >
                {orgName.orgName}
            </div>
        </div>
    );
}
