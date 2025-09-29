// Pages
export const Page = {
    dashboard: 0,
    requests: 1,
    issues: 2,
    feedback: 3,
    settings: 4,
};

export const currentPage = $state({ page: Page.dashboard});