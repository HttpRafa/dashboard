<script lang="ts">
    import { Page, currentPage } from "../page/page.svelte.ts";
</script>

<header class="flex items-center justify-between px-6 py-4 h-16 bg-zinc-800">
    <!-- Left side -->
    <div class="flex items-center space-x-4">
        <h1 class="text-3xl font-semibold text-gray-200">Homelab Services</h1>
        <img
            src="icons/icon.png"
            alt="Icon"
            class="w-10 h-10 object-cover rounded-full hidden sm:block"
        />
    </div>

    <!-- Right side -->
    <div class="flex items-center space-x-4">
        <!-- User Avatar -->
        <button
            onclick={() => {
                currentPage.page = Page.settings;
            }}
            class="w-8 h-8 rounded-full overflow-hidden border-2 border-gray-700 cursor-pointer"
        >
            <img
                src="https://avatars.githubusercontent.com/u/60099368"
                alt="User Avatar"
                class="w-full h-full object-cover"
            />
        </button>
        <!-- Notification Bell 
        <button class="relative text-zinc-400 hover:text-emerald-500 transition cursor-pointer">
            <span class="material-symbols-outlined align-middle"
                >notifications</span
            >
            <span
                class="absolute top-0 right-0 block h-2 w-2 rounded-full bg-red-500"
            ></span>
        </button>-->
        <!-- System Icon 
        <button class="text-zinc-400 hover:text-emerald-500 transition cursor-pointer">
            <span class="material-symbols-outlined align-middle">info</span>
        </button>-->
    </div>
</header>

<style>
</style>
