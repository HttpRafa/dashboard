<script lang="ts">
</script>

<!-- Main Container -->
<div class="p-4 sm:p-6 lg:p-8">
    <!-- Header Section -->
    <header>
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-white">Issues</h1>
            <p class="text-slate-300 mt-1">
                View your reported issues and report new issues
            </p>
        </div>
        <div class="flex justify-between items-center mb-4">
            <button
                class="bg-red-500/65 hover:bg-red-700/65 text-white font-semibold py-2 px-6 rounded-lg transition cursor-pointer"
            >
                Report issue
            </button>
            <button
                class="flex items-center bg-zinc-600/30 hover:bg-zinc-600/70 text-white font-semibold py-2 px-4 rounded-lg transition cursor-pointer"
            >
                <span>Filter</span>
                <span class="text-lg ml-1 material-symbols-outlined"
                    >expand_more</span
                >
            </button>
        </div>
    </header>

    <!-- Issues List -->
    <div class="space-y-3">
        <!-- Table Header -->
        <div
            class="hidden md:grid grid-cols-12 gap-4 px-4 py-2 text-lg font-bold text-white"
        >
            <div class="col-span-12 md:col-span-5">Issue</div>
            <div class="col-span-12 md:col-span-3">Service</div>
            <div class="col-span-12 md:col-span-2">Status</div>
            <div class="col-span-12 md:col-span-2 text-right">Date</div>
        </div>

        <!-- Issue Item 1 -->
        <div
            class="bg-zinc-600/30 p-4 rounded-lg shadow-lg grid grid-cols-12 gap-4 items-center hover:bg-zinc-600/40 transition"
        >
            <!-- Issue Info -->
            <div class="col-span-12 md:col-span-5 flex items-center space-x-4">
                <div
                    class="bg-zinc-800 w-12 h-12 flex items-center justify-center rounded-full"
                >
                    <span class="text-green-400 material-symbols-outlined"
                        >sports_esports</span
                    >
                </div>
                <div>
                    <p class="font-semibold text-white">
                        Add "Vault Hunter" to Game Servers (Valheim) List
                    </p>
                </div>
            </div>
            <!-- Service Info -->
            <div class="col-span-12 md:col-span-3 flex items-center space-x-3">
                <img
                    src="https://placehold.co/32x32/334155/FFFFFF?text=GS"
                    alt="Game Servers"
                    class="w-8 h-8 rounded-md"
                />
                <div>
                    <p class="font-medium text-white">Game Servers</p>
                    <p class="text-sm text-slate-400">Pelican</p>
                </div>
            </div>
            <!-- Status -->
            <div class="col-span-12 md:col-span-2">
                <span
                    class="inline-flex items-center bg-green-500/10 text-green-400 text-xs font-semibold px-3 py-1 rounded-full"
                >
                    <span
                        class="w-2 h-2 bg-green-400 rounded-full mr-2 animate-pulse"
                    ></span>
                    Resolved
                </span>
            </div>
            <!-- Date -->
            <div class="col-span-12 md:col-span-2 text-right">
                <p class="text-sm font-semibold text-white">
                    29.09.2025 at 05:00 PM
                </p>
            </div>
        </div>

        <!-- Issue Item 2 -->
        <div
            class="bg-zinc-600/30 p-4 rounded-lg shadow-lg grid grid-cols-12 gap-4 items-center hover:bg-zinc-600/40 transition"
        >
            <!-- Issue Info -->
            <div class="col-span-12 md:col-span-5 flex items-center space-x-4">
                <div
                    class="bg-zinc-800 w-12 h-12 flex items-center justify-center rounded-full"
                >
                    <span class="text-blue-400 material-symbols-outlined"
                        >photo_library</span
                    >
                </div>
                <div>
                    <p class="font-semibold text-white">
                        JPG Additional Storage for Photos
                    </p>
                </div>
            </div>
            <!-- Service Info -->
            <div class="col-span-12 md:col-span-3 flex items-center space-x-3">
                <img
                    src="https://placehold.co/32x32/334155/FFFFFF?text=PG"
                    alt="Photo Gallery"
                    class="w-8 h-8 rounded-md"
                />
                <div>
                    <p class="font-medium text-white">Photo Gallery</p>
                    <p class="text-sm text-slate-400">Immich</p>
                </div>
            </div>
            <!-- Status -->
            <div class="col-span-12 md:col-span-2">
                <span
                    class="inline-flex items-center bg-blue-500/10 text-blue-400 text-xs font-semibold px-3 py-1 rounded-full"
                >
                    <span
                        class="w-2 h-2 bg-blue-400 rounded-full mr-2 animate-pulse"
                    ></span>
                    Investigating
                </span>
            </div>
            <!-- Date -->
            <div class="col-span-12 md:col-span-2 text-right">
                <p class="text-sm font-semibold text-white">
                    29.09.2025 at 05:00 PM
                </p>
            </div>
        </div>

        <!-- Issue Item 3 -->
        <div
            class="bg-zinc-600/30 p-4 rounded-lg shadow-lg grid grid-cols-12 gap-4 items-center hover:bg-zinc-600/40 transition"
        >
            <!-- Issue Info -->
            <div class="col-span-12 md:col-span-5 flex items-center space-x-4">
                <div
                    class="bg-zinc-800 w-12 h-12 flex items-center justify-center rounded-full"
                >
                    <span class="text-red-400 material-symbols-outlined"
                        >storage</span
                    >
                </div>
                <div>
                    <p class="font-semibold text-white">
                        1TB of Storage Issueed
                    </p>
                </div>
            </div>
            <!-- Service Info -->
            <div class="col-span-12 md:col-span-3 flex items-center space-x-3">
                <img
                    src="https://placehold.co/32x32/334155/FFFFFF?text=CS"
                    alt="Cloud Storage"
                    class="w-8 h-8 rounded-md"
                />
                <div>
                    <p class="font-medium text-white">Cloud Storage</p>
                    <p class="text-sm text-slate-400">Open Cloud</p>
                </div>
            </div>
            <!-- Status -->
            <div class="col-span-12 md:col-span-2">
                <span
                    class="inline-flex items-center bg-red-500/10 text-red-400 text-xs font-semibold px-3 py-1 rounded-full"
                >
                    <span
                        class="w-2 h-2 bg-red-400 rounded-full mr-2 animate-pulse"
                    ></span>
                    Pending
                </span>
            </div>
            <!-- Date -->
            <div class="col-span-12 md:col-span-2 text-right">
                <p class="text-sm font-semibold text-white">
                    29.09.2025 at 05:00 PM
                </p>
            </div>
        </div>
    </div>
</div>

<style>
</style>
