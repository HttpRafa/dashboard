<script lang="ts">
</script>

<div class="flex flex-col md:flex-row justify-center">
    <!-- Left -->
    <div class="md:w-2/3">
        <div class="flex-1 flex flex-col gap-8 p-4">
            <!-- Available Services -->
            <div>
                <h2 class="text-white text-lg font-semibold mb-4">
                    Available Services
                </h2>
                <div class="grid grid-cols-2 gap-4 mb-4">
                    <!-- Game Servers -->
                    <div
                        class="bg-emerald-400/65 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://pelican.dev/"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/pelican.png"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >Game Servers</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Pelican)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined animate-pulse"
                                >ecg_heart</span
                            >
                        </div>
                        <a
                            href="/"
                            class="mt-2 bg-emerald-800/40 text-white rounded py-2 px-4 text-sm hover:bg-emerald-800 transition"
                            >Manage servers</a
                        >
                    </div>
                    <!-- Vaultwarden -->
                    <div
                        class="bg-emerald-400/65 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://github.com/dani-garcia/vaultwarden"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/vaultwarden.svg"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >Password Manager</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Vaultwarden)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined animate-pulse"
                                >ecg_heart</span
                            >
                        </div>
                        <a
                            href="/"
                            class="mt-2 bg-emerald-800/40 text-white rounded py-2 px-4 text-sm hover:bg-emerald-800 transition"
                            >Manage passwords</a
                        >
                    </div>
                    <!-- Open Cloud -->
                    <div
                        class="bg-zinc-500/30 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://opencloud.eu/"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/opencloud.png"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >Cloud Storage</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Open Cloud)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined"
                                >heart_broken</span
                            >
                        </div>
                        <span
                            class="mt-2 bg-red-600/20 text-gray-200 rounded py-2 px-4 text-sm"
                            >Service offline</span
                        >
                    </div>
                    <!-- Immich -->
                    <div
                        class="bg-emerald-400/65 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://immich.app/"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/immich.svg"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >Photo Gallery</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Immich)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined animate-pulse"
                                >ecg_heart</span
                            >
                        </div>
                        <a
                            href="/"
                            class="mt-2 bg-emerald-800/40 text-white rounded py-2 px-4 text-sm hover:bg-emerald-800 transition"
                            >Manage photos</a
                        >
                    </div>
                    <!-- MinIO -->
                    <div
                        class="bg-zinc-500/30 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://www.min.io/"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/minio.png"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >S3 Storage</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(MinIO)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined"
                                >heart_broken</span
                            >
                        </div>
                        <span
                            class="mt-2 bg-blue-500/20 text-gray-200 rounded py-2 px-4 text-sm"
                            >Maintenace until 10:00 AM</span
                        >
                    </div>
                    <!-- Homebox -->
                    <div
                        class="bg-emerald-400/65 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://homebox.software/"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/homebox.svg"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >Inventory Management</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Homebox)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined animate-pulse"
                                >ecg_heart</span
                            >
                        </div>
                        <a
                            href="/"
                            class="mt-2 bg-emerald-800/40 text-white rounded py-2 px-4 text-sm hover:bg-emerald-800 transition"
                            >Manage inventory</a
                        >
                    </div>
                    <!-- Grafana -->
                    <div
                        class="bg-emerald-400/65 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://grafana.com/"
                                    class="w-8 h-8 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/grafana.jpeg"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >Observability Server</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Grafana)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined animate-pulse"
                                >ecg_heart</span
                            >
                        </div>
                        <a
                            href="/"
                            class="mt-2 bg-emerald-800/40 text-white rounded py-2 px-4 text-sm hover:bg-emerald-800 transition"
                            >Manage data</a
                        >
                    </div>
                    <!-- Jenkins -->
                    <div
                        class="bg-emerald-400/65 rounded-lg p-4 flex flex-col justify-between"
                    >
                        <div class="flex items-center justify-between mb-2">
                            <div class="flex items-center gap-2">
                                <a
                                    href="https://www.jenkins.io/"
                                    class="w-8 h-11 rounded-full overflow-hidden"
                                >
                                    <img
                                        src="icons/services/jenkins.svg"
                                        alt="Service Avatar"
                                        class="w-full h-full object-cover"
                                    />
                                </a>
                                <div class="flex flex-col mr-2">
                                    <span
                                        class="text-gray-100 text-sm font-semibold"
                                        >CI/CD Server</span
                                    >
                                    <span class="text-gray-200 text-sm"
                                        >(Jenkins)</span
                                    >
                                </div>
                            </div>
                            <span
                                class="text-white text-2xl material-symbols-outlined animate-pulse"
                                >ecg_heart</span
                            >
                        </div>
                        <a
                            href="/"
                            class="mt-2 bg-emerald-800/40 text-white rounded py-2 px-4 text-sm hover:bg-emerald-800 transition"
                            >Manage workflows</a
                        >
                    </div>
                </div>

                <!-- Planned Services -->
                <div class="mt-6">
                    <h3 class="text-white font-semibold mb-2 text-base">
                        Planned Services
                    </h3>
                    <ul
                        class="list-disc list-inside text-gray-300 text-sm space-y-1"
                    >
                        <li>Pi-hole</li>
                        <li>Plex Media Server</li>
                        <li>Jellyfin</li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
    <!-- Right -->
    <div class="md:w-1/3">
        <div class="flex-1 flex flex-col gap-8 p-4">
            <!-- Recent Activity -->
            <div>
                <h2 class="text-white text-lg font-semibold mb-4">
                    Recent Activity
                </h2>
                <div
                    class="bg-zinc-500/30 rounded-lg p-4 min-h-[120px] mb-4 text-gray-200 text-sm flex flex-col gap-2"
                >
                    <span
                        >You requested 1TB of space for Open Cloud (Pending)</span
                    >
                    <span>Game Server “Valheim” requested</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                    <span>System update completed</span>
                </div>
            </div>
            <!-- Quick Actions -->
            <div>
                <h2 class="text-white text-lg font-semibold mb-4">
                    Quick Actions
                </h2>
                <div class="flex gap-2 mb-2">
                    <a
                        href="/"
                        class="flex-1 bg-blue-500/65 hover:bg-blue-700/65 text-white rounded-lg px-6 py-2 transition"
                    >
                        <span class="font-medium align-middle">Request resources</span>
                    </a>
                    <a
                        href="/"
                        class="flex-1 bg-emerald-400/65 hover:bg-emerald-700/65 text-white rounded-lg px-6 py-2 transition"
                    >
                        <span class="font-medium align-middle">Submit feedback</span>
                    </a>
                </div>
                <div class="flex gap-2 mb-2">
                    <a
                        href="/"
                        class="flex-1 bg-red-400/25 hover:bg-red-400/60 text-white rounded-lg px-6 py-2 transition"
                    >
                        <span class="font-medium align-middle">Report an issue</span>
                    </a>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
</style>
