(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection '("solidity-language-server" "--stdio"))
  :major-modes '(solidity-mode)
  :server-id 'solidity-language-server
  :initialization-options
  '(:solidity-language-server
    (:inlayHints
     (:parameters t
      :gasEstimates t)
     :lint
     (:enabled t
      :severity []
      :only []
      :exclude [])
     :fileOperations
     (:templateOnCreate t
      :updateImportsOnRename t
      :updateImportsOnDelete t)))))
