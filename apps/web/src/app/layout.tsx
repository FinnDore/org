"use client";
import "@/styles/globals.css";

import { Inter } from "next/font/google";

import { ThemeProvider } from "@/components/ui/theme-provider";
import { TRPCReactProvider } from "@/trpc/react";
import { SessionProvider } from "next-auth/react";
import { AxiomWebVitals } from "next-axiom";
import { Nav } from "./(components)/nav";

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
            <body className={`font-sans  ${inter.variable}`}>
                <div className="mx-auto sm:px-12 lg:max-w-screen-lg">
                    <AxiomWebVitals />

                    <ThemeProvider
                        attribute="class"
                        defaultTheme="system"
                        enableSystem
                        disableTransitionOnChange
                    >
                        <TRPCReactProvider>
                            <SessionProvider>
                                <Nav />
                                {children}
                            </SessionProvider>
                        </TRPCReactProvider>
                    </ThemeProvider>
                </div>
            </body>
        </html>
    );
}
