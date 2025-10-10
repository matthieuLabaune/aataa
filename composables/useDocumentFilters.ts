export const useDocumentFilters = () => {
    const activeFilters = useState('activeFilters', () => ({
        type: null as string | null,
        year: null as string | null,
        tags: [] as string[],
        search: '' as string,
    }))

    const viewMode = useState('viewMode', () => 'grid' as 'grid' | 'list')

    const selectedDocument = useState('selectedDocument', () => null as any | null)

    return {
        activeFilters,
        viewMode,
        selectedDocument,
    }
}
