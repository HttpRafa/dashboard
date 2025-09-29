<script lang="ts">
    import { Page, currentPage } from "../page/page.svelte.ts";

    import Icon from "./sidebar/icon.svelte";
</script>

<aside
    class="flex flex-col border-r border-zinc-700 text-zinc-400 w-20 min-h-screen space-y-6"
>
    <!-- Navigation icons -->
    <nav class="flex-1 flex flex-col">
        <!-- Dashboard -->
        <Icon
            icon={"dashboard"}
            text={"Dashboard"}
            selected={currentPage.page == Page.dashboard}
            onClicked={() => {
                currentPage.page = Page.dashboard;
            }}
        />
        <!-- Requests -->
        <Icon
            icon={"assignment_add"}
            text={"Requests"}
            selected={currentPage.page == Page.requests}
            onClicked={() => {
                currentPage.page = Page.requests;
            }}
        />
        <!-- Issues -->
        <Icon
            icon={"pulse_alert"}
            text={"Issues"}
            selected={currentPage.page == Page.issues}
            onClicked={() => {
                currentPage.page = Page.issues;
            }}
        />
        <!-- Feedback -->
        <Icon
            icon={"feedback"}
            text={"Feedback"}
            selected={currentPage.page == Page.feedback}
            onClicked={() => {
                currentPage.page = Page.feedback;
            }}
        />
    </nav>

    <div class="flex flex-col">
        <!-- Identity -->
        <Icon
            icon={"frame_person"}
            text={"Identity"}
            selected={false}
            onClicked={() => {
                window.location.href = "https://auth.httxrafa.dev";
            }}
        />
        <!-- Settings -->
        <Icon
            icon={"settings_account_box"}
            text={"Settings"}
            selected={currentPage.page == Page.settings}
            onClicked={() => {
                currentPage.page = Page.settings;
            }}
        />
    </div>
</aside>

<style>
</style>
