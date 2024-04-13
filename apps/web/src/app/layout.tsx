"use client";
import "@/styles/globals.css";

import { Inter } from "next/font/google";

import { TRPCReactProvider } from "@/trpc/react";
import { SessionProvider } from "next-auth/react";

const inter = Inter({
    subsets: ["latin"],
    variable: "--font-sans",
});

// export const metadata = {
//     title: "Org",
//     description: "Manage your org",
//     icons: [{ rel: "icon", url: "/favicon.ico" }],
// };

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body className={`font-sans ${inter.variable}`}>
                <TRPCReactProvider>
                    <SessionProvider>{children}</SessionProvider>
                </TRPCReactProvider>
            </body>
        </html>
    );
}
