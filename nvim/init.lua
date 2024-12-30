local client = vim.lsp.start_client({
	name = "educationalsp",
	cmd = { os.getenv("PWD") .. "/target/debug/educationalsp" },
})

if not client then
	vim.notify("Bad lsp client setup")
	return
end

vim.api.nvim_create_autocmd("FileType", {
	pattern = "markdown",
	callback = function()
		vim.lsp.buf_attach_client(0, client)
	end,
})
